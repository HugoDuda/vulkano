# vmnl_vulkano

A VMNL Graphics fallback build of [Vulkano 0.35.2](https://github.com/vulkano-rs/vulkano/tree/v0.35.2).
It backports device-loss handling during swapchain and submission future cleanup and preserves
the pending submission after a fence wait times out. See the [upstream proposal](https://github.com/vulkano-rs/vulkano/pull/2856).

The library name is `vulkano`. VMNL uses its own vertex and buffer derives with this build.
The upstream procedural macros use the original Cargo package identity and are omitted here.

The Vulkano code retains its original MIT or Apache-2.0 licensing and attribution.
See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
