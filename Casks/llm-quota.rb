cask "llm-quota" do
  version "0.1.1"
  sha256 :no_check

  name "Llm Quota"
  desc "A compact, high-density dashboard to manage quota limits for LLM accounts"
  homepage "https://github.com/theasmat/llm-quota"

  on_macos do
    arch arm: "aarch64", intel: "x64"
    url "https://github.com/theasmat/llm-quota/releases/download/v#{version}/Llm.Quota_#{version}_#{arch}.dmg"

    app "Llm Quota.app"

    zap trash: [
      "~/Library/Application Support/com.theasmat.llm-quota",
      "~/Library/Caches/com.theasmat.llm-quota",
      "~/Library/Preferences/com.theasmat.llm-quota.plist",
      "~/Library/Saved Application State/com.theasmat.llm-quota.savedState",
    ]

    caveats <<~EOS
      If you encounter the "App is damaged" error, please run the following command:
        sudo xattr -rd com.apple.quarantine "/Applications/Llm Quota.app"

      Or install with the --no-quarantine flag:
        brew install --cask --no-quarantine llm-quota
    EOS
  end

  on_linux do
    arch arm: "aarch64", intel: "amd64"

    url "https://github.com/theasmat/llm-quota/releases/download/v#{version}/Llm.Quota_#{version}_#{arch}.AppImage"
    binary "Llm.Quota_#{version}_#{arch}.AppImage", target: "llm-quota"

    preflight do
      system_command "/bin/chmod", args: ["+x", "#{staged_path}/Llm.Quota_#{version}_#{arch}.AppImage"]
    end
  end
end
