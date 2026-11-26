#!/bin/sh
# Regenerates THIRD-PARTY-LICENSES.md, the license list shipped in the release archives.
# Needs cargo-about and python3.
#
#   cargo install cargo-about --features cli    # once
#   scripts/third-party-licenses.sh
#
# What the list describes: the dependency versions cargo resolves at the moment this script runs
# (those of a local Cargo.lock if there is one). The repository does not commit a Cargo.lock and
# .github/workflows/release.yml resolves again when it builds, so the released program can contain
# versions, or crates, that differ from the list. The release workflow neither regenerates nor
# checks it.
#
# When to run it: after changing dependencies, crates/konoma-mermaid-core/THIRD-PARTY-NOTICES.md or
# licenses/, and again right before tagging a release, after `cargo update` (so that a stale local
# Cargo.lock is not what gets listed). Commit the result before the tag.
#
# Maintained by hand, because neither cargo-about nor scripts/embedded_notices.py can derive them:
# the files in licenses/. Check them when the ratex crates, emojis or konoma-widgets change, and look
# for new embedded data when dependencies are added.
#
# The list is assembled in a temporary file and moved into place only if every step succeeded.
set -eu
cd "$(dirname "$0")/.."

core=crates/konoma-mermaid-core
crates=$(mktemp)
out=$(mktemp ./.third-party-licenses.XXXXXX)
trap 'rm -f "$crates" "$out"' EXIT

cargo about generate --fail --manifest-path crates/konoma-mermaid/Cargo.toml -o "$crates" about.hbs

{
    cat "$crates"
    printf '\n'
    python3 -I scripts/embedded_notices.py Cargo.toml
    cat licenses/katex-fonts.md
    printf '\n'
    cat licenses/gemoji.md
    printf '\n'
    cat licenses/derived-code.md
    printf '\n## Code copied or ported into konoma-mermaid-core\n\n'
    printf 'The paths in this section are relative to `%s/`.\n\n' "$core"
    # Drop the file's own title and push its headings one level down.
    sed -e '1{/^# /d;}' -e 's/^## /### /' "$core/THIRD-PARTY-NOTICES.md"
    printf '\n### Apache License 2.0 (`src/layout/LICENSE-APACHE`)\n\n```text\n'
    cat "$core/src/layout/LICENSE-APACHE"
    printf '```\n'
} > "$out"

chmod 644 "$out"
mv "$out" THIRD-PARTY-LICENSES.md
