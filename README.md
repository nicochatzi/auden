# `auden`

## This is a very early WIP!

### Goals

Yet another thing that looks like a real-time audio engine in Rust.

- Spiritual successor to [rume](https:://github.com/nicochatzi/rume).
- Runtime tooling provided by [aud](https://github.com/nicochatzi/aud).
- Opinionated:
    - On nightly for SIMD, Allocator API, in-place Arc-slice construction...

### Ideas

- [ ] Static Audio Graphs
- [ ] Dynamic Audio Graphs
- [ ] Audio Graph threaded workload balancing
- [ ] Provide higher-level constructs
    - [x] Sample Pools
    - [ ] Plugin adapters
- [ ] State management with recoverable history
