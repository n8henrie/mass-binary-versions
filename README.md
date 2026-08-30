# mass-binary-versions

Vibe-coded with Claude Opus 4.8 on Jun 15, 2026, minimal updates.

`mass-binary-versions` preserves Music Assistant AirPlay binary provenance while GitHub Actions artifacts are still available.

It caches three related things in SQLite:

1. GitHub Actions artifacts from `music-assistant/libraop` and `music-assistant/cliairplay`.
2. SHA-256 hashes of downloaded artifact archives and of each extracted binary.
3. Music Assistant server tags from `music-assistant/server`, with SHA-256 hashes of embedded AirPlay binaries under `music_assistant/providers/airplay/bin/`.

The useful join is:

```text
music-assistant/server tag
  -> embedded AirPlay binary sha256
  -> cached helper artifact extracted file sha256
  -> helper workflow_run.head_sha
  -> compatible libraop or cliairplay commit
```

## Default update

With no subcommand, the program performs the full update:

```bash
mass-binary-versions
```

That concurrently crawls all three sources:

```text
music-assistant/libraop      cliraop-* GitHub Actions artifacts
music-assistant/cliairplay   cliap2-* GitHub Actions artifacts
music-assistant/server       release/tag source zipballs and embedded AirPlay binaries
```

After the concurrent database crawl succeeds, it rewrites the generated compatibility table in `README.md`.

Use a specific database path with either the flag or environment variable:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3
MA_ARTIFACT_DB=mass-binary-versions.sqlite3 mass-binary-versions
```

Explicit equivalent:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3 update-all
```

Useful `update-all` options:

```bash
mass-binary-versions update-all --force
mass-binary-versions update-all --metadata-only
mass-binary-versions update-all --no-readme
mass-binary-versions update-all --readme-limit 100
mass-binary-versions update-all --max-artifact-pages 2 --max-server-tag-pages 2
```

## Music Assistant AirPlay compatibility

This table is generated from the SQLite cache. The GitHub workflow updates it after each daily crawl.

<!-- MA_ARTIFACT_CACHE_TABLE_START -->
| Music Assistant release | server rev | cliairplay commit | libraop commit |
|---|---:|---:|---:|
| `2.11.0.dev2026083003` | [`34e656799767`](https://github.com/music-assistant/server/commit/34e656799767d2a2526759497708cd1ecb8adf7a) | none | none |
| `2.11.0.dev2026082903` | [`b09603cd4efa`](https://github.com/music-assistant/server/commit/b09603cd4efa9ea346d193a3739ddef0d8e75bd5) | none | none |
| `2.11.0.dev2026082805` | [`a9c9297a7ebb`](https://github.com/music-assistant/server/commit/a9c9297a7ebb238d359a30ba9f9da72cd4e824e6) | none | none |
| `2.11.0.dev2026082704` | [`02569ac7b1d8`](https://github.com/music-assistant/server/commit/02569ac7b1d8f5c4bcd28cc2d382e8fb9b3b5771) | none | none |
| `2.11.0b0` | [`1474ae2a16fc`](https://github.com/music-assistant/server/commit/1474ae2a16fc8ddd09921d92daf762d52115a905) | none | none |
| `2.10.1` | [`b666dea6f891`](https://github.com/music-assistant/server/commit/b666dea6f8912b2fc0a258ad15c32290af722a70) | none | none |
| `2.10.0.dev2026082618` | [`acc3af4befc8`](https://github.com/music-assistant/server/commit/acc3af4befc8c8396b0353ba72c1e306a1ce9f0e) | none | none |
| `2.10.0.dev2026082603` | [`c9112a4e50b0`](https://github.com/music-assistant/server/commit/c9112a4e50b00d1a49aacd7a8cc217cc08724869) | none | none |
| `2.10.0.dev2026082516` | [`7cd030ce6567`](https://github.com/music-assistant/server/commit/7cd030ce65674f57641b1db74faac137a3cba76a) | none | none |
| `2.10.0.dev2026082503` | [`9293ab26c446`](https://github.com/music-assistant/server/commit/9293ab26c446b63f2ab1db21526f59dd8edce21b) | none | none |
| `2.10.0.dev2026082414` | [`6c34fcf84661`](https://github.com/music-assistant/server/commit/6c34fcf846610102ffe20d844790434c20c36b50) | none | none |
| `2.10.0.dev2026082403` | [`96ad72a99aa2`](https://github.com/music-assistant/server/commit/96ad72a99aa26427cf24bde62db1d90e57ae637f) | none | none |
| `2.10.0.dev2026082303` | [`3c2bc0ffe4bb`](https://github.com/music-assistant/server/commit/3c2bc0ffe4bb7bbfa33c27e47a6b0d352e53e7f3) | none | none |
| `2.10.0.dev2026082203` | [`8e27c65e0f73`](https://github.com/music-assistant/server/commit/8e27c65e0f733ffbdf307230e4fb3b3fc4d57e1d) | none | none |
| `2.10.0.dev2026082115` | [`de7e679d33e9`](https://github.com/music-assistant/server/commit/de7e679d33e9f6be65461564a8629c3efef9a68f) | none | none |
| `2.10.0.dev2026082103` | [`985b5ec60684`](https://github.com/music-assistant/server/commit/985b5ec606843686ff2ec0742ebefe1e882249e6) | none | none |
| `2.10.0.dev2026082014` | [`b8ec88b8df4f`](https://github.com/music-assistant/server/commit/b8ec88b8df4fae94c54a235ace8a481eb2bef39a) | none | none |
| `2.10.0.dev2026082003` | [`c49927640c97`](https://github.com/music-assistant/server/commit/c49927640c97932b66d3daa6b3fed7c8ade46574) | none | none |
| `2.10.0.dev2026081903` | [`09213beb1532`](https://github.com/music-assistant/server/commit/09213beb1532eff778a8767b303fb4088f04ae44) | none | none |
| `2.10.0.dev2026081800` | [`12ae71400a36`](https://github.com/music-assistant/server/commit/12ae71400a36a6b2b332a8e5bc23642139aaa8f7) | none | none |
| `2.10.0.dev2026081703` | [`73b5d846f7b3`](https://github.com/music-assistant/server/commit/73b5d846f7b3c4c09b9f6bef6b8fdd3f79104e74) | none | none |
| `2.10.0.dev2026081603` | [`2a668f7a5dd8`](https://github.com/music-assistant/server/commit/2a668f7a5dd80c8b1a50df5948fe7825a56c4fa0) | none | none |
| `2.10.0.dev2026081503` | [`cc5db4f38ca2`](https://github.com/music-assistant/server/commit/cc5db4f38ca2a3ea6277e87a53b145f8c628d987) | none | none |
| `2.10.0.dev2026081417` | [`8a3b624c9598`](https://github.com/music-assistant/server/commit/8a3b624c959828ae48af338ee507363316cae296) | none | none |
| `2.10.0.dev2026081403` | [`d073dcba1702`](https://github.com/music-assistant/server/commit/d073dcba1702b1ed9139eac93d09da9c52bdc054) | none | none |
| `2.10.0.dev2026081315` | [`1888d0794bd9`](https://github.com/music-assistant/server/commit/1888d0794bd9b815050963441b1f67abefb83f94) | none | none |
| `2.10.0.dev2026081303` | [`591f5bb03b50`](https://github.com/music-assistant/server/commit/591f5bb03b50f4c50d6cc415e621776ee6f84054) | none | none |
| `2.10.0.dev2026081203` | [`d5fcd47351fe`](https://github.com/music-assistant/server/commit/d5fcd47351fed5aaaf08068d48d44acc365e7b00) | none | none |
| `2.10.0.dev2026081103` | [`a91504084610`](https://github.com/music-assistant/server/commit/a91504084610a817212c17174662cf73a4829bd9) | none | none |
| `2.10.0.dev2026081100` | [`5cb6634a3fb6`](https://github.com/music-assistant/server/commit/5cb6634a3fb64adc84488b7f80ac39d033ee3714) | none | none |
| `2.10.0.dev2026081018` | [`154f1326f218`](https://github.com/music-assistant/server/commit/154f1326f218880258f063660e75a9d0a00bb1e4) | none | none |
| `2.10.0.dev2026081003` | [`e4a502dd0626`](https://github.com/music-assistant/server/commit/e4a502dd062691987a716f1cd8467cbcb33b3144) | none | none |
| `2.10.0.dev2026080903` | [`4c3cfd52910b`](https://github.com/music-assistant/server/commit/4c3cfd52910ba8ad0e27e9d6fa607960d34187ba) | none | none |
| `2.10.0.dev2026080823` | [`336cfe8d5aaf`](https://github.com/music-assistant/server/commit/336cfe8d5aaf7ec6fecba3283e30b606f143bf81) | none | none |
| `2.10.0.dev2026080803` | [`106e6f74ed79`](https://github.com/music-assistant/server/commit/106e6f74ed79680247542ad77fd62d321942e518) | none | none |
| `2.10.0.dev2026080704` | [`34387e3eff9d`](https://github.com/music-assistant/server/commit/34387e3eff9da3dcd8f0968abee3e866ed8e31f1) | none | none |
| `2.10.0.dev2026080604` | [`7edcf5b3609b`](https://github.com/music-assistant/server/commit/7edcf5b3609bf722b8df58445152ebc39d8abef1) | none | none |
| `2.10.0.dev2026080504` | [`5904ce9ba2e1`](https://github.com/music-assistant/server/commit/5904ce9ba2e1a2120274d01d48e0701e6373d051) | none | none |
| `2.10.0.dev2026080404` | [`8313e856667a`](https://github.com/music-assistant/server/commit/8313e856667a77b83c667e5070d40a1a33f8da6d) | none | none |
| `2.10.0.dev2026080304` | [`d653365f8105`](https://github.com/music-assistant/server/commit/d653365f8105666fd9a99bd3a4e6d075cb43725b) | none | none |
| `2.10.0.dev2026080201` | [`91ec08924ea7`](https://github.com/music-assistant/server/commit/91ec08924ea7bc0032fc19f2cbf4015ca54ef669) | none | none |
| `2.10.0.dev2026080101` | [`38129f2f5820`](https://github.com/music-assistant/server/commit/38129f2f582097be1d61f2e5d494515e9c0db519) | none | none |
| `2.10.0.dev2026073104` | [`8471eb51cbcb`](https://github.com/music-assistant/server/commit/8471eb51cbcb7a77fbbafc94e088be73b921d684) | none | none |
| `2.10.0.dev2026073004` | [`ae811753d480`](https://github.com/music-assistant/server/commit/ae811753d480f48bb9dd2a7f1d0d380ffb1bc72c) | none | none |
| `2.10.0.dev2026072904` | [`0baf36c3dfce`](https://github.com/music-assistant/server/commit/0baf36c3dfce63439ea8d177962f860a01bcf2e1) | none | none |
| `2.10.0.dev2026072804` | [`a16cdb27bc1b`](https://github.com/music-assistant/server/commit/a16cdb27bc1bb86a6022b94905dd86d71e7f3e92) | none | none |
| `2.10.0.dev2026072715` | [`be0bc094ef58`](https://github.com/music-assistant/server/commit/be0bc094ef583acfea18805090c9518a3481f4ad) | none | none |
| `2.10.0.dev2026072700` | [`fa17ef799957`](https://github.com/music-assistant/server/commit/fa17ef79995776b836e0e2d9ce13256c189a456a) | none | none |
| `2.10.0.dev2026072613` | [`5578a06bd4fd`](https://github.com/music-assistant/server/commit/5578a06bd4fd5a96f1b887a1b6476ffcdf19e0d0) | none | none |
| `2.10.0.dev2026072604` | [`345fd3b37901`](https://github.com/music-assistant/server/commit/345fd3b37901272cead53a13bb14a411f56ff8b7) | none | none |
| `2.10.0.dev2026072601` | [`08997d3eeb13`](https://github.com/music-assistant/server/commit/08997d3eeb13414f1727621ca48ac8b3a4c05c26) | none | none |
| `2.10.0.dev2026072519` | [`0d47524afaaa`](https://github.com/music-assistant/server/commit/0d47524afaaa905ce5b00156e19c3a0fc437ea32) | none | none |
| `2.10.0.dev2026072510` | [`a087405a28d2`](https://github.com/music-assistant/server/commit/a087405a28d2c0991803dbd9c037dc76fd05a631) | none | none |
| `2.10.0.dev2026072509` | [`197ef9281a36`](https://github.com/music-assistant/server/commit/197ef9281a36dfd818ae88f207288974e2eaebb7) | none | none |
| `2.10.0.dev2026072506` | [`2f90fef13f5f`](https://github.com/music-assistant/server/commit/2f90fef13f5fd0fb43b683d99e805e8272d03db3) | none | none |
| `2.10.0.dev2026072406` | [`d044b96fd807`](https://github.com/music-assistant/server/commit/d044b96fd807610586292e59b8c0757521f66aed) | none | none |
| `2.10.0.dev2026072403` | [`ecbf0c76dcbf`](https://github.com/music-assistant/server/commit/ecbf0c76dcbf89c9e3a1c4944feb4262bb4dcd15) | none | none |
| `2.10.0.dev2026072304` | [`f24b970d5d82`](https://github.com/music-assistant/server/commit/f24b970d5d82daeca1f12b7dff5224a4e9a33a1d) | none | none |
| `2.10.0.dev2026072215` | [`d6dc8ad5f735`](https://github.com/music-assistant/server/commit/d6dc8ad5f735f79a9d67259c02490b8aa6687daa) | none | none |
| `2.10.0.dev2026072207` | [`8787b49638af`](https://github.com/music-assistant/server/commit/8787b49638af6d1b743b947c72919f07dab27462) | none | none |
| `2.10.0.dev2026072204` | [`a547551cf18e`](https://github.com/music-assistant/server/commit/a547551cf18e688948013066bee89552c3a88a43) | none | none |
| `2.10.0.dev2026072105` | [`b57a3ff99e1e`](https://github.com/music-assistant/server/commit/b57a3ff99e1e4baec78c12603031fbc754cf0cc3) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026072006` | [`154513958093`](https://github.com/music-assistant/server/commit/15451395809395bcd36ff9e311d97d8e92cfdadc) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071905` | [`83742c84582a`](https://github.com/music-assistant/server/commit/83742c84582ae75acf6cbd38831f4a0bfaf7c157) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071717` | [`75651d361506`](https://github.com/music-assistant/server/commit/75651d3615068367a7357be81794a358399eb10f) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071705` | [`294f1c0293be`](https://github.com/music-assistant/server/commit/294f1c0293be31c863a04186540fc5170f0d46ae) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071614` | [`afcb08a8e620`](https://github.com/music-assistant/server/commit/afcb08a8e62022f8b8ae044f22894a37e352248a) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071607` | [`92e11272fe1a`](https://github.com/music-assistant/server/commit/92e11272fe1ad6d67b287176037a45429919bc2f) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071605` | [`a74e2af4941e`](https://github.com/music-assistant/server/commit/a74e2af4941e5eddb1e8dc55a13c3b3886a67bb1) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071511` | [`9942285d6cfb`](https://github.com/music-assistant/server/commit/9942285d6cfbac98d46fdfc1da300c7a2b87cf82) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071509` | [`8127c5f23d90`](https://github.com/music-assistant/server/commit/8127c5f23d90e52b60a0da2fc4a8e5798f72d8d9) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071505` | [`a2f14eb9653a`](https://github.com/music-assistant/server/commit/a2f14eb9653afce41fa756dd23e81d09909ec637) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071405` | [`467fe4167c6a`](https://github.com/music-assistant/server/commit/467fe4167c6a40e67db899a7ed39503371d43583) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071222` | [`84b450b2dae2`](https://github.com/music-assistant/server/commit/84b450b2dae2a79bf48519d4b204aa1517b1b0f6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071206` | [`cf190805b153`](https://github.com/music-assistant/server/commit/cf190805b153a0be4f02ffe5f9eddd02a091b330) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
<!-- MA_ARTIFACT_CACHE_TABLE_END -->

`missing` means the Music Assistant tag has an embedded binary hash, but the local cache has not seen a matching helper artifact file hash yet. This usually means the helper artifact expired before the crawler downloaded it, or the relevant workflow artifact has not been crawled yet.

## Build with Nix

```bash
nix develop
cargo generate-lockfile
cargo build --locked
```

After `Cargo.lock` exists, the flake package can be built directly:

```bash
nix build
nix run . -- --help
```

The included GitHub workflow runs `cargo generate-lockfile` on the first update and commits `Cargo.lock`, so a repo created from this project becomes fully packageable after its first successful run.

## Authentication

Use a token for better rate limits and artifact download access:

```bash
export GITHUB_TOKEN=github_pat_...
```

For the included GitHub workflow, `${{ github.token }}` is enough for these public repositories. The workflow grants `actions: read` and `contents: write`.

## Targeted helper artifact crawl

The default no-subcommand update is preferred. For targeted helper artifact work:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3 crawl
```

Equivalent explicit form:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  crawl \
  --repo music-assistant/libraop \
  --repo music-assistant/cliairplay \
  --artifact-prefix cliraop- \
  --artifact-prefix cliap2-
```

Only store artifact metadata, without downloading archives:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3 crawl --metadata-only
```

Force re-download of already-cached artifacts:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3 crawl --force
```

## Targeted Music Assistant server tag crawl

Crawl all server tags and hash embedded AirPlay binaries:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3 crawl-server-tags
```

Crawl only one tag:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  crawl-server-tags \
  --tag 2.8.7
```

The crawler records every embedded binary matching these names:

```text
music_assistant/providers/airplay/bin/cliap2-*
music_assistant/providers/airplay/bin/cliraop-*
```

`cliap2-*` is associated with `music-assistant/cliairplay`; `cliraop-*` is associated with `music-assistant/libraop`.

## Resolve a Music Assistant tag

Rust CLI join:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  resolve-server-tag 2.8.7
```

SQLite example script:

```bash
scripts/resolve-mass-tag.sh mass-binary-versions.sqlite3 2.8.7
```

The output includes the Music Assistant tag, Music Assistant server revision, embedded binary name/hash, matching helper artifact, and helper `head_sha`.

## Update the README table only

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  update-readme \
  --path README.md \
  --limit 75
```

Print the table without editing:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  update-readme \
  --stdout
```

## GitHub automation

The workflow at `.github/workflows/update-cache.yml` runs:

```text
on:
  schedule: daily
  workflow_dispatch: manual
```

It uses `DeterminateSystems/determinate-nix-action@v3`, builds in the Nix dev shell, runs the default full concurrent update, checkpoints/vacuums SQLite, and commits these files when they changed:

```text
Cargo.lock
README.md
mass-binary-versions.sqlite3
```

## Other queries

Hash and look up a local binary from a Music Assistant checkout:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  lookup-file \
  music_assistant/providers/airplay/bin/cliraop-linux-x86_64
```

Look up a raw binary SHA-256:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  lookup-hash f8aeb4a508e203ee93a6eab2a8caa424602d0cfc0c44cf4bfa94112e113f01b6
```

Look up a GitHub artifact/archive digest:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  lookup-archive 13bcbc38a82ea5cde6553a5b172bcc26d96859eedc930af7fe1e4bc75ae4725c
```

List cached rows for a helper commit:

```bash
mass-binary-versions \
  --db mass-binary-versions.sqlite3 \
  list-files \
  --repo music-assistant/libraop \
  --head-sha 556d89ee4ae4422a4d15e6e16401e2656d830aa4
```

Print counts:

```bash
mass-binary-versions --db mass-binary-versions.sqlite3 stats
```

## SQLite tables

The main tables are:

```text
artifacts              one row per GitHub Actions artifact
artifact_files         one row per extracted artifact file
server_tags            one row per Music Assistant server tag
server_tag_binaries    one row per embedded AirPlay binary in a server tag
```

Manual join query:

```sql
select
  st.tag_name as mass_tag,
  st.commit_sha as mass_revision,
  b.helper_package,
  b.binary_name,
  b.file_sha256 as embedded_binary_sha256,
  a.head_sha as helper_commit,
  a.artifact_name,
  a.artifact_id,
  a.run_id,
  a.created_at as artifact_created_at
from server_tag_binaries b
join server_tags st
  on st.repo = b.repo
 and st.tag_name = b.tag_name
left join artifact_files f
  on f.file_sha256 = b.file_sha256
 and f.repo = b.helper_repo
left join artifacts a
  on a.repo = f.repo
 and a.artifact_id = f.artifact_id
where st.repo = 'music-assistant/server'
  and st.tag_name = '2.8.7'
order by b.helper_package, b.binary_name, a.created_at desc;
```

## Development

```bash
nix develop
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test
```
