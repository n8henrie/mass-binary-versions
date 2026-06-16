#!/usr/bin/env bash

set -Eeuf -o pipefail

usage() {
  cat << 'USAGE'
Usage:
  scripts/resolve-mass-tag.sh [DB_PATH] MASS_TAG
  scripts/resolve-mass-tag.sh MASS_TAG

Examples:
  scripts/resolve-mass-tag.sh mass-binary-versions.sqlite3 2.8.7
  scripts/resolve-mass-tag.sh 2.8.7
USAGE
}

main() {
  if [[ $# -eq 1 ]]; then
    db=${MA_ARTIFACT_DB:-mass-binary-versions.sqlite3}
    tag=$1
  elif [[ $# -eq 2 ]]; then
    db=$1
    tag=$2
  else
    usage >&2
    exit 2
  fi

  tag_literal=$(printf '%s' "${tag}" | sed "s/'/''/g")

  sqlite3 -header -column "${db}" << SQL
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
  and st.tag_name = '${tag_literal}'
order by b.helper_package, b.binary_name, a.created_at desc;
SQL
}
main "$@"
