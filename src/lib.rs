#![feature(allocator_api)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod buffer;
pub mod file;
pub mod play;
pub mod plot;
pub mod pool;
