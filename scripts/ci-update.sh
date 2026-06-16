#!/usr/bin/env bash

set -Eeuf -o pipefail
set -x

main() {
  git config user.name "n8henrie/mass-binary-versions[bot]"
  git config user.email "august-pox-armful@duck.com"
  git add Cargo.lock README.md mass-binary-versions.sqlite3

  if git diff --cached --quiet; then
    echo "No cache updates to commit."

    LAST_COMMIT_DATE=$(git log -1 --format="%ct")
    CURRENT_DATE=$(date "+%s")
    TIME_DIFFERENCE=$((CURRENT_DATE - LAST_COMMIT_DATE))
    DAYS_AGO=$((TIME_DIFFERENCE / (60 * 60 * 24)))
    echo "Last commit is '${DAYS_AGO}' days ago"
    if [[ "${DAYS_AGO}" -gt 30 ]]; then
      MESSAGE="No cache updates to commit."
      echo "${MESSAGE}"
      git commit --allow-empty -m "${MESSAGE}"
    fi
  else
    git commit -m "Update Music Assistant AirPlay provenance cache"
  fi
  git push || true
}
main "$@"
