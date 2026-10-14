# GPUI Fast packages for Pioneer

This branch contains the three patched 0.1.7 packages used by Pioneer. Their
published manifests are preserved, and their source files follow the source branch
recorded below.
Other GPUI Fast packages remain on crates.io, as before.

The upstream source patch is on [`fix/composition-input`](https://github.com/pioneerdotai/gpui-fast/tree/fix/composition-input),
commit `cd5692f5fb83df0d362315d55ed182a454214b0e`. It is based on upstream
`d599978f0ecf794b16cb6ec0b5875f818b822507`.

The package names differ from the upstream workspace (`gpui` versus `gpui-fast`),
so this branch keeps the existing published manifests for Cargo consumers. It
contains no application-specific renderer or additional browser engine.

Distributed under Apache-2.0; see LICENSE.
