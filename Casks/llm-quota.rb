cask "llm-quota" do
  version "4.2.2"
  sha256 :no_check

  url "https://github.com/theasmat/llm-quota/releases/download/v#{version}/Llm%20Quota_#{version}_universal.dmg"
  name "Llm Quota"
  desc "A compact, high-density dashboard to manage quota limits for LLM accounts"
  homepage "https://github.com/theasmat/llm-quota"

  app "Llm Quota.app"

  postflight do
    # Remove the quarantine attribute to avoid macOS Gatekeeper warnings
    system_command "xattr",
                   args: ["-cr", "#{appdir}/Llm Quota.app"],
                   sudo: false,
                   must_succeed: false
  end

  zap trash: [
    "~/Library/Application Support/com.lbjlaq.antigravity-tools",
    "~/Library/Caches/com.lbjlaq.antigravity-tools",
    "~/Library/Preferences/com.lbjlaq.antigravity-tools.plist",
    "~/Library/Saved Application State/com.lbjlaq.antigravity-tools.savedState",
    "~/Library/WebKit/com.lbjlaq.antigravity-tools"
  ]
end
