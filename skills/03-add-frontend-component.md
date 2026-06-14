---
name: add-frontend-component
description: >
  Guide for adding a new UI component to the Llm Quota React frontend.
  Covers component structure, Zustand state, Tauri invoke calls, TailwindCSS
  styling, and i18n translation strings.
  Trigger: "add UI", "add component", "new page", "add button", "add panel",
  "frontend feature", "new view", "display data", "add settings".
---

# How to Add a Frontend UI Component

The frontend is React 18 + TypeScript + TailwindCSS + Zustand. This guide covers the complete flow from new component to visible feature.

---

## Directory Structure

```
src/
├── components/
│   ├── accounts/       # Account list, account cards
│   ├── common/         # Shared reusable components (buttons, modals, etc.)
│   ├── dashboard/      # Main quota dashboard views
│   ├── layout/         # App shell / window frame
│   ├── navigation/     # Tab bars, nav items
│   └── sidebar/        # Sidebar panels
├── pages/              # Top-level page components (routed views)
├── stores/             # Zustand global state stores
├── services/           # Tauri invoke wrappers
├── locales/            # i18n translation JSON files
└── types/              # TypeScript types
```

---

## Step 1 — Create the Component File

Place new components in the most relevant subfolder:
- Shared/generic → `src/components/common/MyComponent.tsx`
- Account-specific → `src/components/accounts/MyComponent.tsx`
- Dashboard widget → `src/components/dashboard/MyComponent.tsx`

Basic template:
```tsx
import React from 'react';
import { useTranslation } from 'react-i18next';

interface MyComponentProps {
  title: string;
  onAction?: () => void;
}

export const MyComponent: React.FC<MyComponentProps> = ({ title, onAction }) => {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col gap-2 p-3 rounded-lg bg-surface">
      <h3 className="text-sm font-semibold text-primary">{t('myComponent.title')}</h3>
      <p className="text-xs text-muted">{title}</p>
      {onAction && (
        <button
          onClick={onAction}
          className="btn-primary"
        >
          {t('myComponent.action')}
        </button>
      )}
    </div>
  );
};
```

---

## Step 2 — Add Translation Strings

Open `src/locales/en.json` (or whichever locale files exist) and add keys:
```json
{
  "myComponent": {
    "title": "My Feature",
    "action": "Do Something"
  }
}
```

Add the same keys to all other locale files (e.g., `zh.json`) with translated values.

---

## Step 3 — Read Data from a Zustand Store

Stores live in `src/stores/`. To read shared state:
```tsx
import { useAccountStore } from '@/stores/accountStore';

// Inside your component:
const { accounts, currentAccount } = useAccountStore();
```

To update state, call a store action:
```tsx
const { setCurrentAccount } = useAccountStore();
setCurrentAccount(account);
```

Do **not** call Tauri commands directly from components — use a service (see Step 4).

---

## Step 4 — Call Backend via a Service

Services in `src/services/` wrap Tauri `invoke()` calls:

```typescript
// src/services/myFeatureService.ts
import { invoke } from '@tauri-apps/api/core';
import type { MyData } from '@/types';

export const fetchMyData = (accountId: string): Promise<MyData> =>
  invoke<MyData>('my_tauri_command', { accountId });
```

Use it in your component with `useEffect` or an event handler:
```tsx
import { fetchMyData } from '@/services/myFeatureService';

const handleClick = async () => {
  try {
    const data = await fetchMyData(currentAccount.id);
    // update local state or store
  } catch (err) {
    console.error(err);
  }
};
```

---

## Step 5 — Add the Component to a Page or Layout

Import and render the component wherever it belongs:
```tsx
import { MyComponent } from '@/components/common/MyComponent';

// Inside an existing page/layout component:
<MyComponent title="Hello" onAction={handleClick} />
```

---

## Step 6 — Styling Rules

- Use **TailwindCSS utility classes** — no inline styles.
- Use existing color tokens from `tailwind.config.js` (e.g., `bg-surface`, `text-primary`, `text-muted`).
- For dark/light mode, rely on CSS variables already defined — Tauri's `set_window_theme` command handles this.
- Micro-animations: use `transition-all duration-200` for hover effects.

---

## Checklist

- [ ] Component file created in correct `src/components/` subfolder
- [ ] Props typed with a TypeScript interface
- [ ] All user-visible strings go through `useTranslation()` / `t()`
- [ ] Translation keys added to all locale files in `src/locales/`
- [ ] Tauri calls wrapped in a service in `src/services/` (not called directly)
- [ ] State reads/writes go through Zustand stores
- [ ] Component imported and rendered in the correct page/layout
- [ ] Styling uses TailwindCSS classes with design tokens from `tailwind.config.js`
