#!/bin/sh
# Write the release version into Cargo.toml and Cargo.lock.
# Called by semantic-release (.releaserc.json, prepare step): set-version.sh 1.4.1
set -eu

version="${1:?usage: set-version.sh <version>}"
case "$version" in
  *[!0-9A-Za-z.+-]*) echo "set-version.sh: invalid version '$version'" >&2; exit 1 ;;
esac

# Only the version line of the [package] section changes.
sed -i "/^\[package\]/,/^\[/ s/^version = \".*\"\$/version = \"$version\"/" Cargo.toml
sed -n '/^\[package\]/,/^\[/p' Cargo.toml | grep -qx "version = \"$version\"" ||
  { echo "set-version.sh: [package] version not updated in Cargo.toml" >&2; exit 1; }

# Refresh the lock entry of this crate only: --workspace leaves every dependency
# at its locked version. It reads the crates.io index, so it needs the network.
cargo update --workspace

crate=$(sed -n '/^\[package\]/,/^\[/ s/^name = "\(.*\)"$/\1/p' Cargo.toml)
grep -A1 -x "name = \"$crate\"" Cargo.lock | grep -qx "version = \"$version\"" ||
  { echo "set-version.sh: $crate not at $version in Cargo.lock" >&2; exit 1; }
