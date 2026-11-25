# vmnl_vulkano

A fork of [Vulkano 0.35.2](https://github.com/vulkano-rs/vulkano/tree/v0.35.2) with targeted
GPU future and swapchain cleanup fixes for [VMNL Graphics](https://github.com/VMNL/vmnl-graphics).

[crates.io](https://crates.io/crates/vmnl_vulkano) · [API documentation](https://docs.rs/vmnl_vulkano/latest/vulkano/)

## Use in Cargo

```toml
[dependencies]
vulkano = { package = "vmnl_vulkano", version = "=0.35.2-vmnl.3" }
```

The library is imported as `vulkano`. The upstream `Vertex` and `BufferContents` derives
target the original Cargo package and are omitted here. VMNL provides its own derives.

## Release history

Each version includes the fixes from earlier versions in this release line.

| Version | Changes | Upstream PR |
| --- | --- | --- |
| [0.35.2-vmnl.3](https://crates.io/crates/vmnl_vulkano/0.35.2-vmnl.3) | Updates the package README and homepage. | |
| [0.35.2-vmnl.2](https://crates.io/crates/vmnl_vulkano/0.35.2-vmnl.2) | Destroys a newly created swapchain if image enumeration fails. | [#2857](https://github.com/vulkano-rs/vulkano/pull/2857) |
| [0.35.2-vmnl.1](https://crates.io/crates/vmnl_vulkano/0.35.2-vmnl.1) | Handles device loss during GPU future cleanup and keeps the submission pending after a fence wait times out. | [#2856](https://github.com/vulkano-rs/vulkano/pull/2856) |

## License

The Vulkano code retains its original MIT or Apache-2.0 licensing and attribution.
See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
