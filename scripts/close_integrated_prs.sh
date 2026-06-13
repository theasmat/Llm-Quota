#!/bin/bash

#  v4.0.3  PR 
#  GitHub CLI: brew install gh && gh auth login

REPO="lbjlaq/Antigravity-Manager"
VERSION="v4.0.3"

# 
THANK_YOU_MESSAGE="！🎉

 PR  ${VERSION} 。

：
- README.md 
- 

 Antigravity Tools ！

---

Thank you for your contribution! 🎉

The changes from this PR have been manually integrated into ${VERSION}.

The updates are documented in:
- README.md changelog
- Contributors list

Thank you again for your support of the Antigravity Tools project!"

echo "================================================"
echo " ${VERSION}  PR"
echo "================================================"
echo ""

# PR ： "PR||"
PRS_LIST=(
    "825|IamAshrafee|[Internationalization] Device Fingerprint Dialog localization"
    "822|Koshikai|[Japanese] Add missing translations and refine terminology",
    "798|vietnhatthai|[Translation Fix] Correct spelling error in Vietnamese settings",
    "846|lengjingxu|[]  Token ",
    "949|lbjlaq|Streaming chunks order fix",
    "950|lbjlaq|[Fix] Remove redundant code and update README",
    "973|Mag1cFall|fix:  Windows "
)

#  GitHub CLI 
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI "
    echo ""
    echo " GitHub CLI:"
    echo "  brew install gh"
    echo ""
    echo ":"
    echo "  gh auth login"
    echo ""
    exit 1
fi

# 
if ! gh auth status &> /dev/null; then
    echo "❌  GitHub CLI"
    echo ""
    echo ":"
    echo "  gh auth login"
    echo ""
    exit 1
fi

echo "✅ GitHub CLI "
echo ""

#  PR
for item in "${PRS_LIST[@]}"; do
    PR_NUM=$(echo "$item" | cut -d'|' -f1)
    AUTHOR=$(echo "$item" | cut -d'|' -f2)
    TITLE=$(echo "$item" | cut -d'|' -f3)
    
    echo "----------------------------------------"
    echo " PR #${PR_NUM}: ${TITLE}"
    echo ": @${AUTHOR}"
    echo "----------------------------------------"
    
    # 
    echo "📝 ..."
    gh pr comment ${PR_NUM} --repo ${REPO} --body "${THANK_YOU_MESSAGE}"
    
    if [ $? -eq 0 ]; then
        echo "✅ "
    else
        echo "❌ "
        continue
    fi
    
    #  PR
    echo "🔒  PR..."
    gh pr close ${PR_NUM} --repo ${REPO} --comment " ${VERSION}， PR。"
    
    if [ $? -eq 0 ]; then
        echo "✅ PR #${PR_NUM} "
    else
        echo "❌ PR #${PR_NUM} "
    fi
    
    echo ""
    sleep 2  #  API 
done

echo "================================================"
echo "✅  PR ！"
echo "================================================"
echo ""
echo "："
echo "https://github.com/${REPO}/pulls?q=is%3Apr+is%3Aclosed"
