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
| `2.10.0.dev2026071105` | [`7eed30d54021`](https://github.com/music-assistant/server/commit/7eed30d54021ac4cbd76d39238a7e545007bb030) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071023` | [`470c18a4ae5b`](https://github.com/music-assistant/server/commit/470c18a4ae5b0187e0238f018f043d76e27f5207) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026071006` | [`76b03228586e`](https://github.com/music-assistant/server/commit/76b03228586ea7067562619c0cfa09651b04f3cf) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070906` | [`7571fc1ad44d`](https://github.com/music-assistant/server/commit/7571fc1ad44d56c37c3d47d1bd3aa4e41a5854f6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070805` | [`2da6ba21f3ff`](https://github.com/music-assistant/server/commit/2da6ba21f3ffe9c7d703cbf5f0298c7fc2f5df24) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070706` | [`bd10da75b9e8`](https://github.com/music-assistant/server/commit/bd10da75b9e8895ba44c0a3053a6ad0391e9fada) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070607` | [`85439f5cfd5f`](https://github.com/music-assistant/server/commit/85439f5cfd5fbdfbe36096d57838ac115597d6b3) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070506` | [`f671dd0bcbcc`](https://github.com/music-assistant/server/commit/f671dd0bcbcc977a97378905a2357185f124e777) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070406` | [`8372f27325cb`](https://github.com/music-assistant/server/commit/8372f27325cbcd320db18b4fc3b47454b53d68f3) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070306` | [`d745664f167d`](https://github.com/music-assistant/server/commit/d745664f167de15bbbeb07c3075ab8d11b4f7cca) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070214` | [`fe988082c7cc`](https://github.com/music-assistant/server/commit/fe988082c7cc79e7427de9a3315b868ec9d4ad1e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070206` | [`4befb820f1d1`](https://github.com/music-assistant/server/commit/4befb820f1d139a9e104d9c3af29eac33b7ee071) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026070107` | [`b772801da481`](https://github.com/music-assistant/server/commit/b772801da4818112cb54dee9a84e59e63ed83ef5) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026063006` | [`029c0ba6768c`](https://github.com/music-assistant/server/commit/029c0ba6768cd9c1f051ec0986d198f31ac17147) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062908` | [`7b83a3ede78f`](https://github.com/music-assistant/server/commit/7b83a3ede78fe147f4aadb96988a7136b08322e9) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062815` | [`fbe4d1954068`](https://github.com/music-assistant/server/commit/fbe4d19540683e5526a118c5fe473d04ed46f5fd) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062814` | [`08649fbd675d`](https://github.com/music-assistant/server/commit/08649fbd675d6e173d9cb29e3622ea9f909daacc) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062807` | [`23f5cf0b11fb`](https://github.com/music-assistant/server/commit/23f5cf0b11fb79ce11985949712415f2e78e9ca0) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062706` | [`2472b0c3d1af`](https://github.com/music-assistant/server/commit/2472b0c3d1af2018c762ad2c4e5fb760e01a2412) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062622` | [`9f13e1e54d0c`](https://github.com/music-assistant/server/commit/9f13e1e54d0c0b3dc59b957d1f9a75df8e009b59) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062606` | [`6057196e3ef9`](https://github.com/music-assistant/server/commit/6057196e3ef9b45f1b61294c14d5033d8360cb56) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062506` | [`857aafd95fe6`](https://github.com/music-assistant/server/commit/857aafd95fe65cf0a83c2bd3cf363ef60e5f9286) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062406` | [`d653c388111c`](https://github.com/music-assistant/server/commit/d653c388111c71832d34cfd6a8ec968fd2f43610) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
| `2.10.0.dev2026062306` | [`00e76ad9b27c`](https://github.com/music-assistant/server/commit/00e76ad9b27c1660e27e386567faac6e89be0673) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19)<br>[`81a4413abf12`](https://github.com/music-assistant/cliairplay/commit/81a4413abf1254f1045f7cfa26c1543276598d3c) | missing (0/3 hashes) |
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
