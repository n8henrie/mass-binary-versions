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
| `2.11.0.dev2026101703` | [`bacc51b36f1d`](https://github.com/music-assistant/server/commit/bacc51b36f1d0a53f97742c90b43a09febe009e4) | none | none |
| `2.11.0.dev2026101603` | [`b6df1547d0ed`](https://github.com/music-assistant/server/commit/b6df1547d0edfd0c8d7c51d879050dc99a8caeff) | none | none |
| `2.11.0.dev2026101504` | [`15003cae786a`](https://github.com/music-assistant/server/commit/15003cae786a704959ce4f3933aea30329545cf0) | none | none |
| `2.11.0.dev2026101403` | [`5a00bb73c6fd`](https://github.com/music-assistant/server/commit/5a00bb73c6fdf92078f3de75e8578fbdfa45d866) | none | none |
| `2.11.0.dev2026101303` | [`5f0282adc121`](https://github.com/music-assistant/server/commit/5f0282adc1210ff6a66e026b21e65e4532a3ec7f) | none | none |
| `2.11.0.dev2026101203` | [`db27a42246dc`](https://github.com/music-assistant/server/commit/db27a42246dc90ad62ff82b368c3bbef758c01f7) | none | none |
| `2.11.0.dev2026101103` | [`ca69adb92642`](https://github.com/music-assistant/server/commit/ca69adb926422c494cdc1ce69449475fa9a02598) | none | none |
| `2.11.0.dev2026101003` | [`fef07fcaf0cd`](https://github.com/music-assistant/server/commit/fef07fcaf0cd45f7dda8dcfb95922cd077acef9b) | none | none |
| `2.11.0.dev2026100903` | [`d7bfd48167ed`](https://github.com/music-assistant/server/commit/d7bfd48167ed929c72bbc4721f236d22765aaeb6) | none | none |
| `2.11.0.dev2026100814` | [`73257004745c`](https://github.com/music-assistant/server/commit/73257004745c8b44be6ef43f0001d6230098020d) | none | none |
| `2.11.0.dev2026100803` | [`a813bcd44da9`](https://github.com/music-assistant/server/commit/a813bcd44da9bac651eaabf08e9ede890dfaba53) | none | none |
| `2.11.0.dev2026100703` | [`8df8328fe7c7`](https://github.com/music-assistant/server/commit/8df8328fe7c7f726da68f9c0fd47b26b8b879fa0) | none | none |
| `2.11.0.dev2026100603` | [`41113c424ab6`](https://github.com/music-assistant/server/commit/41113c424ab6b1659f433cb9d8bb0d1ab5f9cbfb) | none | none |
| `2.11.0.dev2026100503` | [`8df7360d9c44`](https://github.com/music-assistant/server/commit/8df7360d9c44d295384924ae2688713849ba232a) | none | none |
| `2.11.0.dev2026100404` | [`2168c3a5db69`](https://github.com/music-assistant/server/commit/2168c3a5db695e6de3878d41043ef76766d96c65) | none | none |
| `2.11.0.dev2026100303` | [`e47afe5e7692`](https://github.com/music-assistant/server/commit/e47afe5e7692c20b9d43574e5f0aee48968fd2e4) | none | none |
| `2.11.0.dev2026100203` | [`6772e14a330d`](https://github.com/music-assistant/server/commit/6772e14a330d7bc26be932998c01b3c8c880b6dc) | none | none |
| `2.11.0.dev2026100103` | [`87be09685a10`](https://github.com/music-assistant/server/commit/87be09685a108d6183eaff98a94f3cedb636217e) | none | none |
| `2.11.0.dev2026093003` | [`f80e6abaad97`](https://github.com/music-assistant/server/commit/f80e6abaad97ca75fd8bb8b5eadf8ae88ada9cea) | none | none |
| `2.11.0.dev2026092903` | [`5e4fab448111`](https://github.com/music-assistant/server/commit/5e4fab44811110d88c006396457aa1def73cd054) | none | none |
| `2.11.0.dev2026092803` | [`1dfe8cc351da`](https://github.com/music-assistant/server/commit/1dfe8cc351daaadd283686f8add95a62aab4b692) | none | none |
| `2.11.0.dev2026092703` | [`f09136859e24`](https://github.com/music-assistant/server/commit/f09136859e240fc7859160e186c2e2186e917715) | none | none |
| `2.11.0.dev2026092603` | [`57b80e26feff`](https://github.com/music-assistant/server/commit/57b80e26feffdd88f1ce0ba2227025895d1c16b7) | none | none |
| `2.11.0.dev2026092503` | [`736daec6d67f`](https://github.com/music-assistant/server/commit/736daec6d67f8bfd3570b1bb568cf795d87e926f) | none | none |
| `2.11.0.dev2026092403` | [`0faf4cd102e4`](https://github.com/music-assistant/server/commit/0faf4cd102e480f137d413878189bcbe6d9d8b4b) | none | none |
| `2.11.0.dev2026092303` | [`9a4261bdd774`](https://github.com/music-assistant/server/commit/9a4261bdd774e701000d476c5f9c2b0f636d0991) | none | none |
| `2.11.0.dev2026092203` | [`9e311eb84aba`](https://github.com/music-assistant/server/commit/9e311eb84aba0a940bdfbf7433d5a29c07bab1b6) | none | none |
| `2.11.0.dev2026092103` | [`89916585da24`](https://github.com/music-assistant/server/commit/89916585da240b85e7dff243408e0ab67e1352cf) | none | none |
| `2.11.0.dev2026092003` | [`a05c7cfe737e`](https://github.com/music-assistant/server/commit/a05c7cfe737e2af117fea11fde6b17e72402f931) | none | none |
| `2.11.0.dev2026091803` | [`4bba1b337f89`](https://github.com/music-assistant/server/commit/4bba1b337f8919ea4eb4ee71ca25d70cbe4a429e) | none | none |
| `2.11.0.dev2026091703` | [`ceae2b59a1fe`](https://github.com/music-assistant/server/commit/ceae2b59a1fe75d6c82c3fb66eaa1aff54dbe1f5) | none | none |
| `2.11.0.dev2026091603` | [`d3d5480dadd7`](https://github.com/music-assistant/server/commit/d3d5480dadd7282da9551415c47e279a94c23eea) | none | none |
| `2.11.0.dev2026091503` | [`db21f47a67c0`](https://github.com/music-assistant/server/commit/db21f47a67c0d4f0b96c9ae2859ad133dac34873) | none | none |
| `2.11.0.dev2026091403` | [`ee38f522a1eb`](https://github.com/music-assistant/server/commit/ee38f522a1eb07c5d359566cc8c171538c89b5e2) | none | none |
| `2.11.0.dev2026091303` | [`c4ad89d9ddbf`](https://github.com/music-assistant/server/commit/c4ad89d9ddbfb76dbb677b0fe5a8342468f2d096) | none | none |
| `2.11.0.dev2026091203` | [`d36fd067a248`](https://github.com/music-assistant/server/commit/d36fd067a2485beb2fdfc391b297f0148ee3853f) | none | none |
| `2.11.0.dev2026091103` | [`e3d7ad076e36`](https://github.com/music-assistant/server/commit/e3d7ad076e3696e818c4bcb64e089e448109bd65) | none | none |
| `2.11.0.dev2026091003` | [`ecbe385ff06c`](https://github.com/music-assistant/server/commit/ecbe385ff06c93f07c9cfd9256c79633f44367cc) | none | none |
| `2.11.0.dev2026090903` | [`8278c7a56ed1`](https://github.com/music-assistant/server/commit/8278c7a56ed198445ee03183031c149f642dacc2) | none | none |
| `2.11.0.dev2026090803` | [`fe1d15d65186`](https://github.com/music-assistant/server/commit/fe1d15d65186ec3fd62661162ecd35aeb3778e7e) | none | none |
| `2.11.0.dev2026090503` | [`52d52ee8d6bf`](https://github.com/music-assistant/server/commit/52d52ee8d6bff777b7502047e4dafba91b8adbb6) | none | none |
| `2.11.0.dev2026090403` | [`fc452e3db502`](https://github.com/music-assistant/server/commit/fc452e3db50269334f0763d5884e26ffe9d084ec) | none | none |
| `2.11.0.dev2026090304` | [`9942087a0666`](https://github.com/music-assistant/server/commit/9942087a0666d22b6c87d8a0cd57aa54eadc96c9) | none | none |
| `2.11.0.dev2026090213` | [`85e9b31abffb`](https://github.com/music-assistant/server/commit/85e9b31abffb77fadf77d0559649104b42d17e6e) | none | none |
| `2.11.0.dev2026090203` | [`a07be8eaa2f0`](https://github.com/music-assistant/server/commit/a07be8eaa2f0a960ed1c059556cb3a31cfea5dc1) | none | none |
| `2.11.0.dev2026090103` | [`3084b2bceabb`](https://github.com/music-assistant/server/commit/3084b2bceabbc61ff10f115975a5bf8e6658d6fe) | none | none |
| `2.11.0.dev2026083003` | [`34e656799767`](https://github.com/music-assistant/server/commit/34e656799767d2a2526759497708cd1ecb8adf7a) | none | none |
| `2.11.0.dev2026082903` | [`b09603cd4efa`](https://github.com/music-assistant/server/commit/b09603cd4efa9ea346d193a3739ddef0d8e75bd5) | none | none |
| `2.11.0.dev2026082805` | [`a9c9297a7ebb`](https://github.com/music-assistant/server/commit/a9c9297a7ebb238d359a30ba9f9da72cd4e824e6) | none | none |
| `2.11.0.dev2026082704` | [`02569ac7b1d8`](https://github.com/music-assistant/server/commit/02569ac7b1d8f5c4bcd28cc2d382e8fb9b3b5771) | none | none |
| `2.11.0b5` | [`949bf248dfe8`](https://github.com/music-assistant/server/commit/949bf248dfe892189991b9f05c2fd38152b1b1d5) | none | none |
| `2.11.0b4` | [`96e2c390c4c3`](https://github.com/music-assistant/server/commit/96e2c390c4c380458e856f7217805b5b2584e704) | none | none |
| `2.11.0b3` | [`36cd4719c029`](https://github.com/music-assistant/server/commit/36cd4719c029db85f9b0205d169817c854b6dad7) | none | none |
| `2.11.0b2` | [`7db7a160882b`](https://github.com/music-assistant/server/commit/7db7a160882b71cc7bb0c1dfc4b6a86c4ebc4876) | none | none |
| `2.11.0b1` | [`fac7b7b81562`](https://github.com/music-assistant/server/commit/fac7b7b815626a76fdbb5627773955b3503a62ad) | none | none |
| `2.11.0b0` | [`1474ae2a16fc`](https://github.com/music-assistant/server/commit/1474ae2a16fc8ddd09921d92daf762d52115a905) | none | none |
| `2.10.6` | [`c24866405bd2`](https://github.com/music-assistant/server/commit/c24866405bd2068c25a60e1014e5ef8c3f6445bb) | none | none |
| `2.10.5` | [`452e23745588`](https://github.com/music-assistant/server/commit/452e23745588f01e734298e0a10e556fc140db45) | none | none |
| `2.10.4` | [`e30a4974ba95`](https://github.com/music-assistant/server/commit/e30a4974ba951f38e21bea8d502af3b903df992c) | none | none |
| `2.10.3` | [`3e21f8293fbc`](https://github.com/music-assistant/server/commit/3e21f8293fbcd8710dc2d3c8afc2c94418187cf9) | none | none |
| `2.10.2` | [`243c4561c174`](https://github.com/music-assistant/server/commit/243c4561c1744501a3087101127c339f4269831b) | none | none |
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
