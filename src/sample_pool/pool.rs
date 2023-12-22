use super::{file::*, manifest::*};
use crate::buffer::shared::*;
use crc32fast::Hasher as Crc32Hasher;
use hashbrown::HashMap;
use std::{fs::File, hash::Hash, io, path::Path};

#[derive(Debug)]
pub enum SampleError {
    FormatError,
    FileError,
    InvalidFormat,
    InvalidChannelCount,
    EmptySample,
}

impl From<hound::Error> for SampleError {
    fn from(value: hound::Error) -> Self {
        match value {
            hound::Error::FormatError(_) => SampleError::FormatError,
            hound::Error::IoError(_) => SampleError::FileError,
            hound::Error::Unsupported => SampleError::InvalidFormat,
            _ => todo!(),
        }
    }
}

impl From<std::io::Error> for SampleError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => SampleError::FileError,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SampleId(uuid::Uuid);

#[derive(Default)]
pub struct SamplePool {
    samples: HashMap<SampleId, SharedAudioBuffer, core::hash::BuildHasherDefault<Crc32Hasher>>,
    files: HashMap<SampleId, std::path::PathBuf, core::hash::BuildHasherDefault<Crc32Hasher>>,
}

impl SamplePool {
    pub fn from_manifest(manifest: Manifest) -> Result<Self, SampleError> {
        manifest
            .entries
            .iter()
            .try_fold(Self::default(), |mut pool, entry| {
                pool.add_sample(&entry.path)?;
                Ok(pool)
            })
    }

    pub fn build_manifest(&self) -> Result<Manifest, io::Error> {
        let mut entries = Vec::<ManifestEntry>::with_capacity(self.files.len());
        let mut buffer = Vec::with_capacity(4096);

        for (id, path) in &self.files {
            entries.push(ManifestEntry {
                path: path.clone(),
                size: self.samples[id].size(),
                hash: hash_file_contents(path, &mut buffer)?,
                name: path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or_default()
                    .to_string(),
            });
        }

        Ok(Manifest::new(entries))
    }

    pub fn from_dir(path: impl AsRef<Path>) -> Result<Self, SampleError> {
        let mut pool = Self::default();
        pool.add_samples(path)?;
        Ok(pool)
    }

    pub fn add_samples(&mut self, dir: impl AsRef<Path>) -> Result<Vec<SampleId>, SampleError> {
        let mut ids = Vec::new();
        walk_dir(dir.as_ref(), &mut |path| {
            if path.extension().map_or(false, |ext| ext == "wav") {
                if let Ok(id) = self.add_sample(path) {
                    ids.push(id);
                }
            }
        })?;
        Ok(ids)
    }

    pub fn add_sample(&mut self, file: impl AsRef<Path>) -> Result<SampleId, SampleError> {
        let reader = hound::WavReader::open(file.as_ref())?;
        let spec = reader.spec();

        let samples = match (spec.sample_format, spec.bits_per_sample) {
            (hound::SampleFormat::Float, 32) => load_f32_wav(reader),
            (hound::SampleFormat::Int, 16) => load_i16_wav(reader),
            (hound::SampleFormat::Int, 24) => load_i24_wav(reader),
            _ => return Err(SampleError::InvalidFormat),
        };

        if spec.sample_rate != 48000 {
            log::warn!("expected sample rate 48kHz, got {}Hz", spec.sample_rate)
        }

        let buffer = match spec.channels {
            1 => SharedAudioBuffer::from_mono(samples),
            2 => SharedAudioBuffer::from_stereo_interleaved(samples),
            _ => return Err(SampleError::InvalidChannelCount),
        };

        self.insert_sample(buffer, file)
    }

    pub fn remove_sample(&mut self, id: SampleId) {
        match self.samples.get(&id) {
            Some(sample) => {
                if !sample.is_unique() {
                    todo!("synchronous sample dropping");
                }
            }
            None => return,
        }

        self.samples.remove(&id);
        self.files.remove(&id);
    }

    pub fn samples(&self) -> impl Iterator<Item = (SampleId, SharedAudioBuffer)> + '_ {
        self.samples
            .iter()
            .map(|(id, buffer)| (*id, buffer.clone()))
    }

    pub fn sample(&self, id: SampleId) -> Option<SharedAudioBuffer> {
        self.samples.get(&id).cloned()
    }

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn live_memory(&self) -> usize {
        self.samples.values().map(|b| b.size()).sum()
    }

    fn insert_sample(
        &mut self,
        buffer: SharedAudioBuffer,
        path: impl AsRef<Path>,
    ) -> Result<SampleId, SampleError> {
        if buffer.is_empty() {
            return Err(SampleError::EmptySample);
        }

        let id = SampleId(uuid::Uuid::new_v4());
        self.samples.insert(id, buffer);
        self.files.insert(id, path.as_ref().to_owned());
        Ok(id)
    }
}

#[inline]
fn load_f32_wav(reader: hound::WavReader<io::BufReader<File>>) -> SharedBuffer {
    let num_samples = reader.len() as usize;
    reader
        .into_samples::<f32>()
        .filter_map(Result::ok)
        .fold(Vec::with_capacity(num_samples), |mut output, sample| {
            output.push(sample);
            output
        })
        .into()
}

#[inline]
fn load_i16_wav(reader: hound::WavReader<io::BufReader<File>>) -> SharedBuffer {
    let num_samples = reader.len() as usize;
    reader
        .into_samples::<i16>()
        .filter_map(Result::ok)
        .fold(Vec::with_capacity(num_samples), |mut output, sample| {
            const I16_TO_FLOAT: f32 = 1.0 / i16::MAX as f32;
            output.push(sample as f32 * I16_TO_FLOAT);
            output
        })
        .into()
}

#[inline]
fn load_i24_wav(reader: hound::WavReader<io::BufReader<File>>) -> SharedBuffer {
    let num_samples = reader.len() as usize;
    reader
        .into_samples::<i32>()
        .filter_map(Result::ok)
        .fold(Vec::with_capacity(num_samples), |mut output, sample| {
            const I24_MAX: i32 = (1 << 23) - 1;
            const I24_TO_FLOAT: f32 = 1.0 / I24_MAX as f32;
            output.push(sample as f32 * I24_TO_FLOAT);
            output
        })
        .into()
}
