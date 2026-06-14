---
name: release-and-ci
description: >
  Complete guide to the Llm Quota CI/CD pipeline and release process.
  Covers the GitHub Actions workflow, multi-platform build matrix (macOS universal/x64/aarch64,
  Linux, Windows), automated README/install script updates, and how to cut a new release.
  Trigger: "how to release", "ci pipeline", "github actions", "cut a release",
  "build fails", "new version", "tag release", "ci workflow".
---

# Release & CI Guide

Llm Quota uses GitHub Actions for fully automated multi-platform builds and releases.

---

## Workflow File

`.github/workflows/release.yml`

---

## Trigger

The pipeline runs when a Git tag matching `v*.*.*` is pushed:
```bash
git tag v0.2.0
git push origin v0.2.0
```

---

## Pipeline Overview

```
Tag pushed (v*.*.*)
        │
        ├── [Job: build-macos-universal]  → Builds universal .dmg + .tar.gz
        ├── [Job: build-macos-x64]        → Builds x64 .dmg
        ├── [Job: build-macos-aarch64]    → Builds aarch64 .dmg
        ├── [Job: build-linux]            → Builds .deb + .rpm + .AppImage
        └── [Job: build-windows]          → Builds .exe + .msi
                │
                ▼ (all build jobs complete)
        [Job: publish-release]
          → Uploads all artifacts to GitHub Release
          → Creates release notes
                │
                ▼
        [Job: update-readme]
          → Runs scripts/update_readme_downloads.py
          → Patches install/mac.sh and install/linux.sh with new version
          → Commits + pushes to master
```

---

## How to Cut a New Release

### Step 1 — Bump version in `tauri.conf.json`

File: `src-tauri/tauri.conf.json`
```json
{
  "version": "0.2.0"
}
```

### Step 2 — Commit the version bump
```bash
git add src-tauri/tauri.conf.json
git commit -m "chore: bump version to 0.2.0"
git push origin master
```

### Step 3 — Tag the release
```bash
git tag v0.2.0
git push origin v0.2.0
```

The CI takes over from here. Monitor the run at:
`https://github.com/theasmat/llm-quota/actions`

---

## What CI Does Automatically

After every successful release tag push:

1. **Builds** all platform binaries in parallel.
2. **Publishes** the GitHub Release with all artifacts attached.
3. **Updates `README.md`** download table with the new version's file sizes and links (via `scripts/update_readme_downloads.py`).
4. **Patches `install/mac.sh`** and **`install/linux.sh`** — replaces the hardcoded fallback version (`# CI_UPDATED_VERSION` comment marker).
5. **Commits and pushes** `README.md` + install scripts back to `master`.

---

## The `# CI_UPDATED_VERSION` Pattern

In `install/mac.sh` and `install/linux.sh`, this line uses a special marker:
```bash
RELEASE_VERSION="0.1.1" # CI_UPDATED_VERSION
```

The CI finds this exact pattern via `sed` and replaces it:
```bash
sed -i "s/RELEASE_VERSION=\".*\" # CI_UPDATED_VERSION/RELEASE_VERSION=\"$VER\" # CI_UPDATED_VERSION/g" install/mac.sh
```

**⚠️ Never remove `# CI_UPDATED_VERSION` from that line** or the CI will stop auto-updating it.

---

## Build Matrix Details

| Platform | Runner | Output Files |
|---|---|---|
| macOS universal | `macos-latest` | `Llm.Quota_x.y.z_universal.dmg`, `Llm.Quota_universal.app.tar.gz` |
| macOS x64 | `macos-13` | `Llm.Quota_x.y.z_x64.dmg`, `Llm.Quota_x64.app.tar.gz` |
| macOS aarch64 | `macos-latest` | `Llm.Quota_x.y.z_aarch64.dmg`, `Llm.Quota_aarch64.app.tar.gz` |
| Linux | `ubuntu-22.04` | `.deb` (amd64, arm64), `.rpm` (x86_64, aarch64), `.AppImage` |
| Windows | `windows-latest` | `.exe`, `.msi` |

---

## Troubleshooting Common CI Failures

| Symptom | Likely Cause | Fix |
|---|---|---|
| `cargo build` fails | Rust compilation error introduced | Fix the Rust code and push a new tag |
| `npm install` / `pnpm install` fails | Node dependency lock conflict | Commit updated `pnpm-lock.yaml` |
| Release upload fails with "already exists" | Tag was re-pushed after a failed run | Delete the GitHub Release, delete the tag, re-push |
| README commit fails | Git identity not configured on runner | Check `git config` step in workflow |
| `sed` doesn't update install scripts | `# CI_UPDATED_VERSION` marker removed | Restore the marker comment on the version line |

---

## Checklist for a New Release

- [ ] Version bumped in `src-tauri/tauri.conf.json`
- [ ] `CHANGELOG` or release notes prepared (optional but recommended)
- [ ] All changes committed and pushed to `master`
- [ ] Tag created with `v` prefix (`v0.2.0`, not `0.2.0`)
- [ ] Tag pushed to remote — CI starts automatically
- [ ] CI passes all build jobs (~10-15 min)
- [ ] GitHub Release created with all artifacts
- [ ] `master` branch has updated README + install scripts (auto-committed by CI)
