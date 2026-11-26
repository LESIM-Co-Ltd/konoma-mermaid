"""Prints the notices of third-party data that dependency crates embed in the program.

cargo-about reports the license a crate declares, not the licenses of the fonts, character maps,
color profiles, syntax definitions and other files inside it. The texts are read from the sources
of the crate versions cargo resolves, so they match the crate list. A missing crate or file is an
error: it means this list has to be looked at again.

Used by scripts/third-party-licenses.sh: python3 -I embedded_notices.py <workspace Cargo.toml>
"""

import json
import os
import re
import struct
import subprocess
import sys
import zlib

meta = json.loads(
    subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--manifest-path", sys.argv[1]]
    )
)


def crates(name):
    found = sorted(
        (p["version"], os.path.dirname(p["manifest_path"]))
        for p in meta["packages"]
        if p["name"] == name
    )
    if not found:
        sys.exit(f"embedded_notices: crate {name} is not in the dependency graph")
    return found


def read(path):
    with open(path, encoding="utf-8") as f:
        text = f.read()
    if not text.strip():
        sys.exit(f"embedded_notices: {path} is empty")
    return text


def block(text):
    longest = max((len(m) for m in re.findall(r"`+", text)), default=0)
    fence = "`" * max(3, longest + 1)
    return f"{fence}text\n{text.rstrip(chr(10))}\n{fence}\n"


def section(title, crate, intro, files):
    for version, root in crates(crate):
        print(f"## {title} (`{crate}` {version})\n\n{intro}\n")
        for rel in files:
            print(f"`{rel}`:\n\n{block(read(os.path.join(root, rel)))}")


section(
    "Foxit fonts and a CMYK color profile",
    "hayro-interpret",
    "Embedded stand-ins for the standard PDF fonts (from PDFium) and an ICC profile (CC0).",
    ["assets/README.md", "assets/LICENSE_FOXIT", "assets/CGATS_LICENSE.txt"],
)
section("Adobe CMap resources", "hayro-cmap", "Embedded character maps.", ["assets/LICENSE.txt"])
section(
    "ICC color profiles",
    "hayro-jpeg2000",
    "Embedded color profiles.",
    ["assets/README.md", "assets/LICENSE.txt"],
)
section(
    "Unicode data",
    "regex-syntax",
    "Tables generated from the Unicode Character Database.",
    ["src/unicode_tables/LICENSE-UNICODE"],
)
section("Unicode emoji data", "emojis", "Emoji data from Unicode.", ["LICENSE-UNICODE"])
section(
    "Code from BoringSSL, fiat-crypto and others",
    "ring",
    "Compiled into the crate.",
    ["LICENSE-other-bits", "LICENSE-BoringSSL", "third_party/fiat/LICENSE"],
)
section(
    "Wayland protocol definitions",
    "wayland-protocols",
    "Linux builds contain code generated from these protocol files.",
    ["protocols/COPYING"],
)
for title, crate, rel in [
    ("Wayland core protocol", "wayland-client", "wayland.xml"),
    (
        "ext data control protocol",
        "wayland-protocols",
        "protocols/staging/ext-data-control/ext-data-control-v1.xml",
    ),
    (
        "wlr data control protocol",
        "wayland-protocols-wlr",
        "wlr-protocols/unstable/wlr-data-control-unstable-v1.xml",
    ),
]:
    for version, root in crates(crate):
        notice = re.search(r"<copyright>\n?(.*?)\s*</copyright>", read(os.path.join(root, rel)), re.S)
        if not notice or not notice[1].strip():
            sys.exit(f"embedded_notices: no <copyright> in {crate}/{rel}")
        print(f"## {title} (`{crate}` {version})\n\nThe `<copyright>` element of `{rel}`:\n")
        print(block(notice[1]))


# two-face carries the notices of its syntax definitions and themes as a zlib-compressed bincode
# dump of `Acknowledgements { for_syntaxes: Vec<License>, for_themes: Vec<License> }`, with
# `License { ty: enum, text: String, rel_path: PathBuf }`.
def acknowledgements(path):
    with open(path, "rb") as f:
        data = zlib.decompress(f.read())
    pos = 0

    def take(n):
        nonlocal pos
        if pos + n > len(data):
            sys.exit("embedded_notices: unexpected format of the two-face acknowledgements")
        out = data[pos : pos + n]
        pos += n
        return out

    def u64():
        return struct.unpack("<Q", take(8))[0]

    def string():
        return take(u64()).decode("utf-8")

    groups = []
    for _ in range(2):
        entries = []
        for _ in range(u64()):
            take(4)  # license type
            text = string()
            entries.append((string(), text))
        groups.append(entries)
    if pos != len(data) or not all(groups):
        sys.exit("embedded_notices: unexpected format of the two-face acknowledgements")
    return groups


for version, root in crates("two-face"):
    print(f"## Syntax definitions and themes (`two-face` {version})\n")
    print(
        "Embedded syntax definitions and themes, curated by the `bat` project "
        "(<https://github.com/sharkdp/bat>). These are the notices `two-face` ships in "
        "`generated/acknowledgements_full.bin`, which covers the licenses that require "
        "acknowledgement.\n"
    )
    groups = acknowledgements(os.path.join(root, "generated/acknowledgements_full.bin"))
    for kind, entries in zip(("Syntax", "Theme"), groups):
        for rel_path, text in entries:
            print(f"### {kind}: `{rel_path}`\n\n{block(text)}")
