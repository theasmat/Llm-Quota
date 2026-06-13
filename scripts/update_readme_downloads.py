import os
import re
import sys
import glob

def format_size(size_bytes):
    if size_bytes == 0:
        return "0 B"
    size_mb = size_bytes / (1024 * 1024)
    return f"{size_mb:.2f} MB"

def update_readme():
    tag = os.environ.get("RELEASE_TAG")
    if not tag:
        print("RELEASE_TAG environment variable not set. Exiting.")
        sys.exit(0)

    readme_path = "README.md"
    if not os.path.exists(readme_path):
        print(f"{readme_path} not found.")
        sys.exit(1)

    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    base_url = f"https://github.com/theasmat/llm-quota/releases/download/{tag}"
    
    # We will search for the files in the 'all-artifacts' directory
    artifacts_dir = "all-artifacts"
    
    def get_file_info(pattern, default_url="#", default_size="TBD"):
        search_path = os.path.join(artifacts_dir, pattern)
        matches = glob.glob(search_path)
        if matches:
            filepath = matches[0]
            filename = os.path.basename(filepath)
            size = format_size(os.path.getsize(filepath))
            url = f"{base_url}/{filename}"
            return url, size
        return default_url, default_size

    # Windows
    win_exe_url, win_exe_size = get_file_info("Llm.Quota_*_x64-setup.exe")
    win_msi_url, win_msi_size = get_file_info("Llm.Quota_*_x64_en-US.msi")

    # macOS
    mac_uni_dmg_url, mac_uni_dmg_size = get_file_info("Llm.Quota_*_universal.dmg")
    mac_arm_dmg_url, mac_arm_dmg_size = get_file_info("Llm.Quota_*_aarch64.dmg")
    mac_x64_dmg_url, mac_x64_dmg_size = get_file_info("Llm.Quota_*_x64.dmg")
    
    mac_uni_tar_url, mac_uni_tar_size = get_file_info("Llm.Quota_universal.app.tar.gz")
    mac_arm_tar_url, mac_arm_tar_size = get_file_info("Llm.Quota_aarch64.app.tar.gz")
    mac_x64_tar_url, mac_x64_tar_size = get_file_info("Llm.Quota_x64.app.tar.gz")

    # Linux
    lin_amd_app_url, lin_amd_app_size = get_file_info("Llm.Quota_*_amd64.AppImage")
    lin_arm_app_url, lin_arm_app_size = get_file_info("Llm.Quota_*_aarch64.AppImage")
    
    lin_amd_deb_url, lin_amd_deb_size = get_file_info("Llm.Quota_*_amd64.deb")
    lin_arm_deb_url, lin_arm_deb_size = get_file_info("Llm.Quota_*_arm64.deb")
    
    lin_amd_rpm_url, lin_amd_rpm_size = get_file_info("Llm.Quota-*-1.x86_64.rpm")
    lin_arm_rpm_url, lin_arm_rpm_size = get_file_info("Llm.Quota-*-1.aarch64.rpm")

    new_table = f"""## 🚀 Downloads

Select the appropriate package for the target operating system.

| 🍎 macOS                                                                      | 🪟 Windows                                                       | 🐧 Linux                                                                  |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------- |
| [⬇️ Universal (.dmg)]({mac_uni_dmg_url}) _({mac_uni_dmg_size})_        | [⬇️ x64 (.exe)]({win_exe_url}) _({win_exe_size})_ | [⬇️ amd64 (.AppImage)]({lin_amd_app_url}) _({lin_amd_app_size})_   |
| [⬇️ Apple Silicon (.dmg)]({mac_arm_dmg_url}) _({mac_arm_dmg_size})_    | [⬇️ x64 (.msi)]({win_msi_url}) _({win_msi_size})_ | [⬇️ aarch64 (.AppImage)]({lin_arm_app_url}) _({lin_arm_app_size})_ |
| [⬇️ Intel x64 (.dmg)]({mac_x64_dmg_url}) _({mac_x64_dmg_size})_        |                                                                  | [⬇️ amd64 (.deb)]({lin_amd_deb_url}) _({lin_amd_deb_size})_        |
| [⬇️ Universal (.tar.gz)]({mac_uni_tar_url}) _({mac_uni_tar_size})_     |                                                                  | [⬇️ arm64 (.deb)]({lin_arm_deb_url}) _({lin_arm_deb_size})_        |
| [⬇️ Apple Silicon (.tar.gz)]({mac_arm_tar_url}) _({mac_arm_tar_size})_ |                                                                  | [⬇️ x86_64 (.rpm)]({lin_amd_rpm_url}) _({lin_amd_rpm_size})_       |
| [⬇️ Intel x64 (.tar.gz)]({mac_x64_tar_url}) _({mac_x64_tar_size})_     |                                                                  | [⬇️ aarch64 (.rpm)]({lin_arm_rpm_url}) _({lin_arm_rpm_size})_      |
"""

    pattern = re.compile(r"## 🚀 Downloads.*?(\n---\n)", re.DOTALL)
    
    if pattern.search(content):
        new_content = pattern.sub(new_table + r"\1", content)
        with open(readme_path, "w", encoding="utf-8") as f:
            f.write(new_content)
        print("README.md updated successfully with direct download links.")
    else:
        print("Could not find ## 🚀 Downloads block in README.md")
        sys.exit(1)

if __name__ == "__main__":
    update_readme()
