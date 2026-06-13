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
    
    def get_file_info(pattern):
        # find the actual file matching the pattern
        search_path = os.path.join(artifacts_dir, pattern)
        matches = glob.glob(search_path)
        if matches:
            filepath = matches[0]
            filename = os.path.basename(filepath)
            size = format_size(os.path.getsize(filepath))
            url = f"{base_url}/{filename}"
            return filename, url, size
        return None, None, None

    # Windows
    win_exe_name, win_exe_url, win_exe_size = get_file_info("Llm.Quota_*_x64-setup.exe")
    win_msi_name, win_msi_url, win_msi_size = get_file_info("Llm.Quota_*_x64_en-US.msi")

    # macOS
    mac_uni_dmg_name, mac_uni_dmg_url, mac_uni_dmg_size = get_file_info("Llm.Quota_*_universal.dmg")
    mac_arm_dmg_name, mac_arm_dmg_url, mac_arm_dmg_size = get_file_info("Llm.Quota_*_aarch64.dmg")
    mac_x64_dmg_name, mac_x64_dmg_url, mac_x64_dmg_size = get_file_info("Llm.Quota_*_x64.dmg")
    
    mac_uni_tar_name, mac_uni_tar_url, mac_uni_tar_size = get_file_info("Llm.Quota_universal.app.tar.gz")
    mac_arm_tar_name, mac_arm_tar_url, mac_arm_tar_size = get_file_info("Llm.Quota_aarch64.app.tar.gz")
    mac_x64_tar_name, mac_x64_tar_url, mac_x64_tar_size = get_file_info("Llm.Quota_x64.app.tar.gz")

    # Linux
    lin_amd_app_name, lin_amd_app_url, lin_amd_app_size = get_file_info("Llm.Quota_*_amd64.AppImage")
    lin_arm_app_name, lin_arm_app_url, lin_arm_app_size = get_file_info("Llm.Quota_*_aarch64.AppImage")
    
    lin_amd_deb_name, lin_amd_deb_url, lin_amd_deb_size = get_file_info("Llm.Quota_*_amd64.deb")
    lin_arm_deb_name, lin_arm_deb_url, lin_arm_deb_size = get_file_info("Llm.Quota_*_arm64.deb")
    
    lin_amd_rpm_name, lin_amd_rpm_url, lin_amd_rpm_size = get_file_info("Llm.Quota-*-1.x86_64.rpm")
    lin_arm_rpm_name, lin_arm_rpm_url, lin_arm_rpm_size = get_file_info("Llm.Quota-*-1.aarch64.rpm")

    def make_row(name, url, size, arch):
        if not name: return ""
        return f"| **[{name}]({url})** | {arch} | {size} |\n"

    new_table = "<!-- DOWNLOAD_TABLE_START -->\n"
    new_table += "<!-- The download table will be automatically injected here by GitHub Actions -->\n"
    new_table += "## Downloads\n\n"
    new_table += "Select the appropriate package for the target operating system and architecture.\n\n"
    
    new_table += "### 🪟 Windows\n\n"
    new_table += "| Installer File                         | Architecture | Size    |\n"
    new_table += "| :------------------------------------- | :----------- | :------ |\n"
    new_table += make_row(win_exe_name, win_exe_url, win_exe_size, "x64")
    new_table += make_row(win_msi_name, win_msi_url, win_msi_size, "x64")

    new_table += "\n### 🍎 macOS\n\n"
    new_table += "| Installer / Archive File                | Architecture  | Size    |\n"
    new_table += "| :-------------------------------------- | :------------ | :------ |\n"
    new_table += make_row(mac_uni_dmg_name, mac_uni_dmg_url, mac_uni_dmg_size, "Universal")
    new_table += make_row(mac_arm_dmg_name, mac_arm_dmg_url, mac_arm_dmg_size, "Apple Silicon")
    new_table += make_row(mac_x64_dmg_name, mac_x64_dmg_url, mac_x64_dmg_size, "Intel x64")
    new_table += make_row(mac_uni_tar_name, mac_uni_tar_url, mac_uni_tar_size, "Universal")
    new_table += make_row(mac_arm_tar_name, mac_arm_tar_url, mac_arm_tar_size, "Apple Silicon")
    new_table += make_row(mac_x64_tar_name, mac_x64_tar_url, mac_x64_tar_size, "Intel x64")

    new_table += "\n### 🐧 Linux\n\n"
    new_table += "| Package File                              | Architecture | Size    |\n"
    new_table += "| :---------------------------------------- | :----------- | :------ |\n"
    new_table += make_row(lin_amd_app_name, lin_amd_app_url, lin_amd_app_size, "amd64")
    new_table += make_row(lin_arm_app_name, lin_arm_app_url, lin_arm_app_size, "aarch64")
    new_table += make_row(lin_amd_deb_name, lin_amd_deb_url, lin_amd_deb_size, "amd64")
    new_table += make_row(lin_arm_deb_name, lin_arm_deb_url, lin_arm_deb_size, "arm64")
    new_table += make_row(lin_amd_rpm_name, lin_amd_rpm_url, lin_amd_rpm_size, "x86_64")
    new_table += make_row(lin_arm_rpm_name, lin_arm_rpm_url, lin_arm_rpm_size, "aarch64")
    
    new_table += "<!-- DOWNLOAD_TABLE_END -->"

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
