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
| `2.10.0.dev2026070406` | [`8372f27325cb`](https://github.com/music-assistant/server/commit/8372f27325cbcd320db18b4fc3b47454b53d68f3) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026070306` | [`d745664f167d`](https://github.com/music-assistant/server/commit/d745664f167de15bbbeb07c3075ab8d11b4f7cca) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026070214` | [`fe988082c7cc`](https://github.com/music-assistant/server/commit/fe988082c7cc79e7427de9a3315b868ec9d4ad1e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026070206` | [`4befb820f1d1`](https://github.com/music-assistant/server/commit/4befb820f1d139a9e104d9c3af29eac33b7ee071) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026070107` | [`b772801da481`](https://github.com/music-assistant/server/commit/b772801da4818112cb54dee9a84e59e63ed83ef5) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026063006` | [`029c0ba6768c`](https://github.com/music-assistant/server/commit/029c0ba6768cd9c1f051ec0986d198f31ac17147) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062908` | [`7b83a3ede78f`](https://github.com/music-assistant/server/commit/7b83a3ede78fe147f4aadb96988a7136b08322e9) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062815` | [`fbe4d1954068`](https://github.com/music-assistant/server/commit/fbe4d19540683e5526a118c5fe473d04ed46f5fd) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062814` | [`08649fbd675d`](https://github.com/music-assistant/server/commit/08649fbd675d6e173d9cb29e3622ea9f909daacc) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062807` | [`23f5cf0b11fb`](https://github.com/music-assistant/server/commit/23f5cf0b11fb79ce11985949712415f2e78e9ca0) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062706` | [`2472b0c3d1af`](https://github.com/music-assistant/server/commit/2472b0c3d1af2018c762ad2c4e5fb760e01a2412) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062622` | [`9f13e1e54d0c`](https://github.com/music-assistant/server/commit/9f13e1e54d0c0b3dc59b957d1f9a75df8e009b59) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062606` | [`6057196e3ef9`](https://github.com/music-assistant/server/commit/6057196e3ef9b45f1b61294c14d5033d8360cb56) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062506` | [`857aafd95fe6`](https://github.com/music-assistant/server/commit/857aafd95fe65cf0a83c2bd3cf363ef60e5f9286) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062406` | [`d653c388111c`](https://github.com/music-assistant/server/commit/d653c388111c71832d34cfd6a8ec968fd2f43610) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062306` | [`00e76ad9b27c`](https://github.com/music-assistant/server/commit/00e76ad9b27c1660e27e386567faac6e89be0673) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062108` | [`737284df23f9`](https://github.com/music-assistant/server/commit/737284df23f9fc32ce46c60ba602dcbecbc20cf7) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026062011` | [`f3abfaf164b6`](https://github.com/music-assistant/server/commit/f3abfaf164b6587883264eba3942f099c29949f4) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061808` | [`ec2d4414fe45`](https://github.com/music-assistant/server/commit/ec2d4414fe4513e8beb754050859106b973c29cf) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061708` | [`4c70d55743b6`](https://github.com/music-assistant/server/commit/4c70d55743b62c52679f29d9593412bfbc3f54ba) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061608` | [`7a127eefcd73`](https://github.com/music-assistant/server/commit/7a127eefcd733ff47334617b200dead7df9c59ae) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061509` | [`7f2b0f8e5017`](https://github.com/music-assistant/server/commit/7f2b0f8e5017e9b40597d9e3364c2e7d6643d5c8) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061415` | [`c57e475f3ca2`](https://github.com/music-assistant/server/commit/c57e475f3ca29a1c617205bc60090e2ebc5c071b) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061407` | [`02bfd66d0350`](https://github.com/music-assistant/server/commit/02bfd66d0350b846cf73ffe5daa8a963a885e72f) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061308` | [`eb39dea7dab5`](https://github.com/music-assistant/server/commit/eb39dea7dab576fbb9da9f89b80745fa451d861f) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061307` | [`3dddcc938856`](https://github.com/music-assistant/server/commit/3dddcc938856b16b92fcdf8684700a5d69b6b2c2) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061207` | [`69be0c919a8f`](https://github.com/music-assistant/server/commit/69be0c919a8f015c5c99e4a0351977442ffb19d6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061108` | [`483c07e222d6`](https://github.com/music-assistant/server/commit/483c07e222d673bb29d2d86cdefdae1d5db89a99) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0.dev2026061105` | [`63e397269763`](https://github.com/music-assistant/server/commit/63e397269763db8304564edd832c1bd1942488ad) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0b2` | [`50fe73aac411`](https://github.com/music-assistant/server/commit/50fe73aac411d66f2645fd292235506d08b52b9c) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0b1` | [`880f0402ccc1`](https://github.com/music-assistant/server/commit/880f0402ccc1977c02509a463ea5b91f8e6d2ece) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.10.0b0` | [`e81ecdda8ed4`](https://github.com/music-assistant/server/commit/e81ecdda8ed4079d4af42ce7dc6c98a23e620ad9) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.5` | [`36e1dcea33e4`](https://github.com/music-assistant/server/commit/36e1dcea33e42a182f88258b25944d005c328dd0) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.4` | [`eac299861005`](https://github.com/music-assistant/server/commit/eac2998610055e02313b4a0dbdd2a5b5c6f83715) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.3` | [`e8fc4d1c91ef`](https://github.com/music-assistant/server/commit/e8fc4d1c91ef022beed235b5556080f395fec021) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.2` | [`2dd6b56a6fda`](https://github.com/music-assistant/server/commit/2dd6b56a6fda8f734d70d1fb9f593699567ebdd2) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.1` | [`6e7986b4463b`](https://github.com/music-assistant/server/commit/6e7986b4463b5b8fbfc0bb99548151af3e0a497e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026061014` | [`ac79f5a69e3b`](https://github.com/music-assistant/server/commit/ac79f5a69e3b451ef6b477716141f45167bada8e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026061009` | [`e6260e2f9c56`](https://github.com/music-assistant/server/commit/e6260e2f9c5628b8eb6e0731cbe8e65d4ac75af6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026061007` | [`edb2bab67857`](https://github.com/music-assistant/server/commit/edb2bab67857b1e1fe6042c5d040bf70b4650b4c) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060914` | [`b79a9a42afda`](https://github.com/music-assistant/server/commit/b79a9a42afdaaf7aed1dbea1d3d41a74e7a563c7) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060904` | [`e26bbcd00f13`](https://github.com/music-assistant/server/commit/e26bbcd00f133b3c06a0ded17ec09a6ddeffa237) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060808` | [`30fceb0412f3`](https://github.com/music-assistant/server/commit/30fceb0412f30b52c4b0b483dfde89c5c9943f66) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060708` | [`bebb04bcec1b`](https://github.com/music-assistant/server/commit/bebb04bcec1bb7c96d5ce35e9d3dfcef2bbcd341) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060707` | [`ddaa0202017b`](https://github.com/music-assistant/server/commit/ddaa0202017b2a45d4d925c6a5c32f8dc9d3dccf) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060611` | [`d3073756d34e`](https://github.com/music-assistant/server/commit/d3073756d34e6ac72c0fc5027dc253c0de98346e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060606` | [`a2483d593781`](https://github.com/music-assistant/server/commit/a2483d59378176bf989ce517f39ad808790a012d) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060510` | [`f94184679625`](https://github.com/music-assistant/server/commit/f94184679625db326c08278c7308eb87d3e4d95b) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060507` | [`3c71e0293dd5`](https://github.com/music-assistant/server/commit/3c71e0293dd5b127dbe83fa71faea63530997545) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060407` | [`eb2a0b592396`](https://github.com/music-assistant/server/commit/eb2a0b5923963d36371941e5b683dd14d44fbcae) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060308` | [`5926e37e8433`](https://github.com/music-assistant/server/commit/5926e37e84330073f2c122633c6e9045d74c198a) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060218` | [`a795401ca70f`](https://github.com/music-assistant/server/commit/a795401ca70fcd1e70c008b5584495ba30472e44) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060208` | [`21b21c74cf5c`](https://github.com/music-assistant/server/commit/21b21c74cf5c584acc620e6159da1b2332baf9ce) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026060108` | [`9897c9f1b2ea`](https://github.com/music-assistant/server/commit/9897c9f1b2ea179413268325c65f05a5165d514b) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026053107` | [`b867dd6dee16`](https://github.com/music-assistant/server/commit/b867dd6dee16d8aeba4584a1db4f41c38d009d2d) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026053104` | [`f2e824641845`](https://github.com/music-assistant/server/commit/f2e82464184511b2c3b478d2ede0b200b2ba7b60) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026053007` | [`f3f08a11e1f1`](https://github.com/music-assistant/server/commit/f3f08a11e1f1cdd88c0bfca6469c91939e64643a) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026053006` | [`b5c52d01ed9d`](https://github.com/music-assistant/server/commit/b5c52d01ed9d22cc435628e96b4a6fda4246039e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052906` | [`fe5e67010d5c`](https://github.com/music-assistant/server/commit/fe5e67010d5c97fe59455f0980e3a3937d2eb448) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052818` | [`705304706b5d`](https://github.com/music-assistant/server/commit/705304706b5d70cff3b66b7d04db31ccdc5aa37e) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052806` | [`e2e74af5c314`](https://github.com/music-assistant/server/commit/e2e74af5c3145424d3bb1f2a00206a125b933a23) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052710` | [`3f6b804d9e2e`](https://github.com/music-assistant/server/commit/3f6b804d9e2e18c0eb9f8ac4ea6e767651d11a16) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052707` | [`a5c04476300d`](https://github.com/music-assistant/server/commit/a5c04476300d677d30af19fb908cff6445e3416d) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052619` | [`b667d9c68607`](https://github.com/music-assistant/server/commit/b667d9c686071c521d443eaee4b766d1c1c0d015) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052606` | [`c239ef1117b9`](https://github.com/music-assistant/server/commit/c239ef1117b964ddd9e455bda83e8493ba8dfbe0) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052507` | [`dd018624b237`](https://github.com/music-assistant/server/commit/dd018624b2374297477d464fb4d65225d8b9b7c6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052406` | [`17bdce137ff8`](https://github.com/music-assistant/server/commit/17bdce137ff82921c963daef5ed98fbd5912c251) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052304` | [`7fef658ca0bc`](https://github.com/music-assistant/server/commit/7fef658ca0bcb3415d834cb1005f4664967e72a6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052206` | [`a57035df37e7`](https://github.com/music-assistant/server/commit/a57035df37e7209a0c37fc27290b5e463748597a) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052106` | [`1a1de04d711d`](https://github.com/music-assistant/server/commit/1a1de04d711d642e9d6469be5a013c9f9d2661ba) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052010` | [`502836145e17`](https://github.com/music-assistant/server/commit/502836145e1790f881b1f20b2e17bb621165e5b6) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026052006` | [`0da20f7fa380`](https://github.com/music-assistant/server/commit/0da20f7fa380430c0f6a610d1793fef56c30cf15) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026051906` | [`49467b983319`](https://github.com/music-assistant/server/commit/49467b9833192f73978ed61903457944ab16958f) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026051905` | [`70d7b0ce99b4`](https://github.com/music-assistant/server/commit/70d7b0ce99b4db4ece36378acd55e831be4bc31d) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
| `2.9.0.dev2026051706` | [`85b49a595f2e`](https://github.com/music-assistant/server/commit/85b49a595f2ea4bde98d26c93ebb96498f5bd2fb) | [`3bb927164399`](https://github.com/music-assistant/cliairplay/commit/3bb9271643999696638ee5df421b69bb5112fb32)<br>[`6aeceb49e4e3`](https://github.com/music-assistant/cliairplay/commit/6aeceb49e4e37d044f09be9369b082fc26bcfa19) | missing (0/3 hashes) |
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
