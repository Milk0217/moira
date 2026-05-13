# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root — defines the project's domain language (星盘、七政四馀、二十八宿、神煞等)
- **`docs/adr/`** — read ADRs that touch the area you're about to work in

If any of these files don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront.

## File structure

Single-context repo:

```
/
├── CONTEXT.md
├── docs/adr/
│   └── 0001-flutter-to-tauri-migration.md
└── moira-rs/
    └── ...
```

## Use the glossary's vocabulary

When your output names a domain concept, use the term as defined in `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

Key terms: 星盘 (Xīngpán), 七政四馀 (Qīzhèng Sìyú), 二十八宿 (Èrshíbā Xiù), 神煞 (Shénshà), 星曜, 宫位, JPL DE 星历.

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding.
