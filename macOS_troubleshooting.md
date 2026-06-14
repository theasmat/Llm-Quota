# macOS Installation & Opening Troubleshooting

When downloading and installing **Llm Quota** on macOS outside the Mac App Store, you might encounter warnings such as "App is damaged and can't be opened" or "Cannot be opened because the developer cannot be verified."

## Why does this happen?

macOS includes a security feature called **Gatekeeper**. By default, Gatekeeper only allows you to open apps downloaded from the App Store or those signed by an Apple Developer and notarized by Apple. Because this application is open-source and not currently signed with a paid Apple Developer certificate, Gatekeeper flags it and assigns a "quarantine" attribute to the downloaded file.

## How to solve it

You can bypass this warning using one of the following methods:

### Method 1: The "Right-Click" Override (Recommended)
1. Drag `Llm Quota.app` into your **Applications** folder.
2. Open the **Applications** folder in Finder.
3. **Right-click** (or Control-click) on the `Llm Quota` app.
4. Select **Open** from the context menu.
5. A dialog will appear warning you about the unidentified developer. Click **Open** anyway.
*Note: You only have to do this once. The app will open normally by double-clicking in the future.*

### Method 2: Removing the Quarantine Flag via Terminal
If macOS stubbornly insists the app is "damaged", you can manually clear the quarantine flag.
1. Drag `Llm Quota.app` into your **Applications** folder.
2. Open the **Terminal** app.
3. Run the following command:
   ```bash
   xattr -cr /Applications/Llm\ Quota.app
   ```
4. You can now double-click the app to open it normally.
