use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Debug)]
pub enum StreamError {
    NoDevice,
    Play(cpal::PlayStreamError),
    Pause(cpal::PauseStreamError),
    Build(cpal::BuildStreamError),
}

pub struct Stream {
    stream: cpal::Stream,
}

impl Drop for Stream {
    fn drop(&mut self) {
        if let Err(e) = self.stream.pause() {
            log::error!("failed to stop stream : {e}");
        }
    }
}

impl Stream {
    pub fn launch(
        config: &cpal::StreamConfig,
        callback: impl FnMut(&mut [f32], usize) + Send + 'static,
    ) -> Result<Self, StreamError> {
        let device = default_device()?;
        log::info!("selected device : {}", device.name().unwrap_or_default());

        let stream = launch(&device, config, callback).map_err(StreamError::Build)?;
        stream.play().map_err(StreamError::Play)?;
        Ok(Self { stream })
    }

    pub fn launch_with_timeout(
        config: &cpal::StreamConfig,
        timeout: std::time::Duration,
        callback: impl FnMut(&mut [f32], usize) + Send + 'static,
    ) -> Result<(), StreamError> {
        let device = default_device()?;
        log::info!("selected device : {}", device.name().unwrap_or_default());

        let stream = launch(&device, config, callback).map_err(StreamError::Build)?;

        stream.play().map_err(StreamError::Play).and_then(|_| {
            std::thread::sleep(timeout);
            stream.pause().map_err(StreamError::Pause)
        })
    }
}

fn default_device() -> Result<cpal::Device, StreamError> {
    cpal::default_host()
        .default_output_device()
        .ok_or(StreamError::NoDevice)
}

fn launch(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut callback: impl FnMut(&mut [f32], usize) + Send + 'static,
) -> Result<cpal::Stream, cpal::BuildStreamError> {
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
            callback(output, channels);
        },
        |e| log::error!("error occurred on stream : {e}"),
        None,
    )?;
    Ok(stream)
}
