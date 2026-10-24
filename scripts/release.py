#!/usr/bin/env python3
"""Release/changelog tooling for Ferrugen.

`prepare` generates a release section from commits with git-cliff, creates
`CHANGELOG.md` on the first release, updates the release links and bumps the
workspace version. There is no `[Unreleased]` section; entries are generated
when a release is prepared.

`verify` is the deterministic CI gate used for release tags. It fails when the
tag version, `Cargo.toml` and `CHANGELOG.md` disagree.
"""

from __future__ import annotations

import argparse
import datetime as dt
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HEADER_MARKER = "<!-- git-cliff: end of header -->"
FOOTER_RE = re.compile(r"^\[([^\]]+)\]:\s+(\S+)\s*$")
VERSION_RE = re.compile(r"^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def fail(message: str) -> None:
    print(f"error: {message}", file=sys.stderr)
    raise SystemExit(1)


def normalize_version(value: str) -> str:
    return value[1:] if value.startswith("v") else value


def validate_version(value: str) -> str:
    version = normalize_version(value.strip())
    if not VERSION_RE.match(version):
        fail(f"invalid version '{value}' (expected MAJOR.MINOR.PATCH)")
    return version


def version_tuple(value: str) -> tuple[int, int, int]:
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)", value)
    if not match:
        return (0, 0, 0)
    return tuple(int(part) for part in match.groups())  # type: ignore[return-value]


def version_sort_key(value: str) -> tuple[int, int, int, int]:
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)", value)
    if not match:
        return (0, 0, 0, 0)
    stable = 0 if "-" in value else 1
    return (*tuple(int(part) for part in match.groups()), stable)  # type: ignore[return-value]


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        fail(f"{path} does not exist")
    except OSError as error:
        fail(f"cannot read {path}: {error}")


def write_text(path: Path, content: str) -> None:
    try:
        path.write_text(content, encoding="utf-8")
    except OSError as error:
        fail(f"cannot write {path}: {error}")


def workspace_version(cargo_text: str) -> str:
    in_section = False
    for line in cargo_text.splitlines():
        stripped = line.strip()
        if stripped == "[workspace.package]":
            in_section = True
            continue
        if in_section and stripped.startswith("[") and stripped.endswith("]"):
            break
        if in_section:
            match = re.match(r'^\s*version\s*=\s*"([^"]+)"', line)
            if match:
                return match.group(1)
    fail("cannot find [workspace.package].version in Cargo.toml")


def replace_workspace_version(cargo_text: str, version: str) -> str:
    lines = cargo_text.splitlines(keepends=True)
    in_section = False
    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped == "[workspace.package]":
            in_section = True
            continue
        if in_section and stripped.startswith("[") and stripped.endswith("]"):
            break
        if in_section:
            match = re.match(r'^(\s*version\s*=\s*")([^"]+)(".*)$', line.rstrip("\n"))
            if match:
                suffix = "\n" if line.endswith("\n") else ""
                lines[index] = f'{match.group(1)}{version}{match.group(3)}{suffix}'
                return "".join(lines)
    fail("cannot find [workspace.package].version in Cargo.toml")


def detect_repo_url(repo: Path, cargo_text: str) -> str:
    try:
        remote = subprocess.check_output(
            ["git", "-C", str(repo), "remote", "get-url", "origin"],
            stderr=subprocess.DEVNULL,
            text=True,
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        remote = ""

    if remote.startswith("git@") and ":" in remote:
        host, path = remote[4:].split(":", 1)
        return f"https://{host}/{path.removesuffix('.git')}"
    if remote.startswith("ssh://"):
        match = re.match(r"ssh://git@([^/]+)/(.+)", remote)
        if match:
            return f"https://{match.group(1)}/{match.group(2).removesuffix('.git')}"
    if remote.startswith(("https://", "http://")):
        return remote.removesuffix(".git")

    match = re.search(r'^\s*repository\s*=\s*"([^"]+)"', cargo_text, re.MULTILINE)
    if match:
        return match.group(1).removesuffix(".git")
    fail("cannot determine repository URL; add an origin remote or repository metadata")


def split_changelog(text: str) -> tuple[list[str], str, list[str]]:
    lines = text.splitlines()
    if any(
        line.strip() == "## [Unreleased]"
        or line.strip().startswith("[Unreleased]:")
        for line in lines
    ):
        fail(
            "CHANGELOG.md must not contain an '## [Unreleased]' section or link; "
            "release entries are generated when a release is prepared"
        )

    marker_index = next(
        (index for index, line in enumerate(lines) if HEADER_MARKER in line), None
    )
    first_release_index = next(
        (index for index, line in enumerate(lines) if line.startswith("## [")),
        len(lines),
    )
    header_end = marker_index + 1 if marker_index is not None else first_release_index
    header_lines = lines[:header_end]

    content_after = lines[header_end:]
    footer_start = None
    for index in range(len(content_after) - 1, -1, -1):
        line = content_after[index]
        if FOOTER_RE.match(line.strip()):
            footer_start = index
        elif line.strip() == "":
            continue
        else:
            break

    if footer_start is None:
        content_lines = content_after
        footer_lines: list[str] = []
    else:
        content_lines = content_after[:footer_start]
        footer_lines = content_after[footer_start:]

    content = "\n".join(content_lines).strip()
    return header_lines, content, footer_lines


def render_footer(footer_lines: list[str], repo_url: str, version: str) -> str:
    links: dict[str, str] = {}
    for line in footer_lines:
        match = FOOTER_RE.match(line.strip())
        if match:
            links[match.group(1)] = match.group(2)

    links.pop("Unreleased", None)
    links[version] = f"{repo_url}/releases/tag/v{version}"

    release_keys = sorted(links, key=version_sort_key, reverse=True)
    return "\n".join(f"[{key}]: {links[key]}" for key in release_keys)


def generate_release(repo: Path, version: str) -> tuple[str, str]:
    git_cliff = os.environ.get("GIT_CLIFF", "git-cliff")
    config = repo / "cliff.toml"
    if not config.is_file():
        fail(f"{config} does not exist")

    tags = subprocess.run(
        ["git", "-C", str(repo), "tag", "--list", "v*"],
        text=True,
        capture_output=True,
        check=False,
    )
    if tags.returncode == 0 and not tags.stdout.strip():
        print(
            "warning: no v* tags found; the generated entry includes the full history",
            file=sys.stderr,
        )

    with tempfile.NamedTemporaryFile(
        prefix="ferrugen-changelog-", suffix=".md", delete=False
    ) as output:
        output_path = Path(output.name)

    try:
        try:
            result = subprocess.run(
                [
                    git_cliff,
                    "--config",
                    str(config),
                    "--repository",
                    str(repo),
                    "--unreleased",
                    "--tag",
                    f"v{version}",
                    "--output",
                    str(output_path),
                ],
                text=True,
                capture_output=True,
                check=False,
            )
        except FileNotFoundError:
            fail("git-cliff is not installed; run: cargo install git-cliff --locked")

        if result.returncode != 0:
            detail = result.stderr.strip() or result.stdout.strip()
            fail(f"git-cliff failed: {detail}")

        generated = read_text(output_path)
    finally:
        output_path.unlink(missing_ok=True)

    if "## [Unreleased]" in generated:
        fail("git-cliff produced an '## [Unreleased]' section; check cliff.toml")

    lines = generated.splitlines()
    heading_index = next(
        (index for index, line in enumerate(lines) if line.startswith("## [")), None
    )
    if heading_index is None:
        fail("git-cliff produced no release section; there are no commits for this release")

    header = "\n".join(lines[:heading_index]).strip()
    body = "\n".join(lines[heading_index + 1 :]).strip()
    if not body:
        fail("git-cliff produced an empty release section")
    return header, re.sub(r"\n{3,}", "\n\n", body)


def prepare(args: argparse.Namespace) -> None:
    version = validate_version(args.version)
    release_date = args.date or dt.datetime.now(dt.timezone.utc).date().isoformat()
    if not DATE_RE.match(release_date):
        fail(f"invalid date '{release_date}' (expected YYYY-MM-DD)")
    try:
        dt.date.fromisoformat(release_date)
    except ValueError:
        fail(f"invalid date '{release_date}'")

    repo = Path(args.repo).resolve()
    changelog_path = repo / args.changelog
    cargo_path = repo / args.cargo

    cargo_text = read_text(cargo_path)
    current_version = workspace_version(cargo_text)
    if version_tuple(current_version) > version_tuple(version):
        fail(
            f"new version {version} is lower than the current workspace version {current_version}"
        )

    changelog_exists = changelog_path.is_file()
    if changelog_exists:
        changelog_text = read_text(changelog_path)
        if re.search(rf"^## \[{re.escape(version)}\]", changelog_text, re.MULTILINE):
            fail(f"CHANGELOG.md already contains a release entry for {version}")
        header_lines, previous_content, footer_lines = split_changelog(changelog_text)
        header = "\n".join(header_lines).rstrip()
    else:
        header = ""
        previous_content = ""
        footer_lines = []

    generated_header, release_body = generate_release(repo, version)
    if not header:
        header = generated_header
    if not header:
        fail("git-cliff produced no changelog header; add a header to cliff.toml")

    parts = [
        header,
        "",
        f"## [{version}] - {release_date}",
        "",
        release_body,
        "",
    ]
    if previous_content:
        parts.extend([previous_content, ""])

    repo_url = detect_repo_url(repo, cargo_text)
    parts.append(render_footer(footer_lines, repo_url, version))
    parts.append("")

    new_changelog = "\n".join(parts)
    new_cargo = replace_workspace_version(cargo_text, version)

    write_text(changelog_path, new_changelog)
    write_text(cargo_path, new_cargo)

    action = "Updated" if changelog_exists else "Created"
    print(f"{action} CHANGELOG.md and prepared Cargo.toml for {version} ({release_date}).")
    print("Review the diff, include it in a release PR, then tag after merge:")
    print(f"  git tag -a v{version} -m v{version}")
    print(f"  git push origin v{version}")


def verify(args: argparse.Namespace) -> None:
    version = validate_version(args.version)
    repo = Path(args.repo).resolve()
    changelog_path = repo / args.changelog
    cargo_path = repo / args.cargo

    cargo_version = workspace_version(read_text(cargo_path))
    if cargo_version != version:
        fail(
            f"workspace version is {cargo_version}, but the release tag is {version}"
        )

    changelog_text = read_text(changelog_path)
    if re.search(r"^## \[Unreleased\]", changelog_text, re.MULTILINE):
        fail("CHANGELOG.md must not contain an '## [Unreleased]' section")
    if re.search(r"^\[Unreleased\]:", changelog_text, re.MULTILINE):
        fail("CHANGELOG.md must not contain an '[Unreleased]' link")

    heading = re.search(
        rf"^## \[{re.escape(version)}\] - (?P<date>\d{{4}}-\d{{2}}-\d{{2}})\s*$",
        changelog_text,
        re.MULTILINE,
    )
    if not heading:
        fail(
            f"CHANGELOG.md has no dated entry for {version}; run scripts/release.sh {version}"
        )

    try:
        dt.date.fromisoformat(heading.group("date"))
    except ValueError:
        fail(f"CHANGELOG.md has an invalid release date: {heading.group('date')}")

    first_release = re.search(r"^## \[([^\]]+)\]", changelog_text, re.MULTILINE)
    if not first_release or first_release.group(1) != version:
        actual = first_release.group(1) if first_release else "none"
        fail(
            f"CHANGELOG.md latest release is {actual}, but the tag is {version}"
        )

    print(f"PASS: changelog entry for {version} is present and dated.")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    prepare_parser = subparsers.add_parser(
        "prepare",
        help="generate a release entry (creating CHANGELOG.md) and bump the workspace version",
    )
    prepare_parser.add_argument("version", help="release version, for example 0.2.0")
    prepare_parser.add_argument("--date", help="release date (default: today, UTC)")
    prepare_parser.add_argument("--repo", default=".", help="repository root")
    prepare_parser.add_argument("--changelog", default="CHANGELOG.md")
    prepare_parser.add_argument("--cargo", default="Cargo.toml")
    prepare_parser.set_defaults(func=prepare)

    verify_parser = subparsers.add_parser(
        "verify", help="verify the changelog entry for a release tag"
    )
    verify_parser.add_argument("version", help="release version, for example 0.2.0")
    verify_parser.add_argument("--repo", default=".", help="repository root")
    verify_parser.add_argument("--changelog", default="CHANGELOG.md")
    verify_parser.add_argument("--cargo", default="Cargo.toml")
    verify_parser.set_defaults(func=verify)

    return parser


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
