import os
import re
import sys
import json

def update_readme():
    tag = os.environ.get("RELEASE_TAG")
    if not tag:
        print("RELEASE_TAG environment variable not set. Exiting.")
        sys.exit(0)

    # Clean the tag just in case (e.g., v0.1.0 -> 0.1.0)
    version = tag.lstrip('v')

    # Read package.json to ensure we get the exact version if different from tag
    try:
        with open("package.json", "r") as f:
            pkg = json.load(f)
            version = pkg.get("version", version)
    except:
        pass

    readme_path = "README.md"
    if not os.path.exists(readme_path):
        print(f"{readme_path} not found.")
        sys.exit(1)

    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    base_url = f"https://github.com/theasmat/llm-quota/releases/download/{tag}"
    
    # Filename formats (Tauri standard)
    mac_x64 = f"Llm%20Quota_{version}_x64.dmg"
    mac_arm = f"Llm%20Quota_{version}_aarch64.dmg"
    mac_uni = f"Llm%20Quota_{version}_universal.dmg"
    win_x64 = f"Llm%20Quota_{version}_x64-setup.exe"
    win_arm = f"Llm%20Quota_{version}_arm64-setup.exe"
    linux_x64 = f"llm-quota_{version}_amd64.AppImage"
    linux_arm = f"llm-quota_{version}_aarch64.AppImage"

    new_table = f"""<!-- DOWNLOAD_TABLE_START -->
<!-- The download table will be automatically injected here by GitHub Actions -->
| Architecture / OS | 🍎 macOS | 🪟 Windows | 🐧 Linux |
|:---:|:---:|:---:|:---:|
| **x86_64 (Intel/AMD)** | [⬇️ Download .dmg]({base_url}/{mac_x64}) | [⬇️ Download .exe]({base_url}/{win_x64}) | [⬇️ Download .AppImage]({base_url}/{linux_x64}) |
| **arm64 (Apple Silicon/ARM)** | [⬇️ Download .dmg]({base_url}/{mac_arm}) | [⬇️ Download .exe]({base_url}/{win_arm}) | [⬇️ Download .AppImage]({base_url}/{linux_arm}) |
| **Universal (All)** | [⬇️ Download .dmg]({base_url}/{mac_uni}) | - | - |
<!-- DOWNLOAD_TABLE_END -->"""

    pattern = re.compile(r"<!-- DOWNLOAD_TABLE_START -->.*?<!-- DOWNLOAD_TABLE_END -->", re.DOTALL)
    
    if pattern.search(content):
        new_content = pattern.sub(new_table, content)
        with open(readme_path, "w", encoding="utf-8") as f:
            f.write(new_content)
        print("README.md updated successfully with direct download links.")
    else:
        print("Could not find DOWNLOAD_TABLE_START block in README.md")
        sys.exit(1)

if __name__ == "__main__":
    update_readme()
