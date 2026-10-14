# GPUI Fast packages for Pioneer

This branch contains the three patched 0.1.7 packages used by Pioneer. Their
manifests and source files are unchanged from the locally selected packages.
Other GPUI Fast packages remain on crates.io, as before.

The upstream source patch is on [`fix/composition-input`](https://github.com/pioneerdotai/gpui-fast/tree/fix/composition-input),
commit `44decb4dcd59921bd677a623946d0f5676c203a3`. It is based on upstream
`371769f1ff378bc0bd886440e0f97fc94aaeae5d`.

The package names differ from the upstream workspace (`gpui` versus `gpui-fast`),
so this branch keeps the existing published manifests for Cargo consumers. It
contains no application-specific renderer or additional browser engine.

Distributed under Apache-2.0; see LICENSE.
