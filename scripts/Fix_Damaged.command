#!/bin/bash

# 
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
APP_PATH="$DIR/Antigravity Tools.app"

# 
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}==============================================${NC}"
echo -e "${GREEN}   Antigravity Tools - ${NC}"
echo -e "${GREEN}==============================================${NC}"
echo ""

if [ -d "$APP_PATH" ]; then
    echo "📍 : $APP_PATH"
    echo "🔑  ()..."
    echo ""
    
    # 
    sudo xattr -rd com.apple.quarantine "$APP_PATH"
    
    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}✅ !${NC}"
        echo "。"
        
        #  AppleScript 
        osascript -e 'display notification "，" with title "Antigravity Tools" sound name "Glass"'
    else
        echo ""
        echo -e "${RED}❌ ${NC}"
        echo "，。"
    fi
else
    echo -e "${RED}⚠️  ${NC}"
    echo " 'Antigravity Tools.app'  ( /Applications)。"
fi

echo ""
echo "..."
read -n 1 -s -r -p ""
