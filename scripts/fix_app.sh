#!/bin/bash

APP_PATH="/Applications/Antigravity Tools.app"

echo "🛠️   'Antigravity Tools' ..."

if [ -d "$APP_PATH" ]; then
    echo "📍 : $APP_PATH"
    echo "🔑  (Quarantine Attribute)..."
    
    sudo xattr -rd com.apple.quarantine "$APP_PATH"
    
    if [ $? -eq 0 ]; then
        echo "✅ ！。"
    else
        echo "❌ ，。"
    fi
else
    echo "⚠️  ， '/Applications' 。"
    echo "   ，: sudo xattr -rd com.apple.quarantine /path/to/app"
fi
