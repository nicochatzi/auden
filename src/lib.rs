#![feature(allocator_api)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod buffer;
pub mod dsp;
pub mod sample_pool;

#[cfg(feautre = "std")]
pub mod stream;

#[cfg(feautre = "std")]
pub mod sample_pool;
