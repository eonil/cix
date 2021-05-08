CIX
===
Eonil, 2021.

CI tools collection.

This is a collection of small and compositable tools to build your own custom CI tool.
Focused on Apple platform program CI at first. Maybe expand to other platforms.


Design Choices
--------------
- Support both human and machine mode interface.
  - This can be controlled by environment variable. (setting `CIX_INTERFACE_MODE` to `human` or `machine`)
  - `machine` mode means JSON through std in/out.


Feature: Missing dSYM Detection
-------------------------------
```bash
cix v0 detect-missing-dsym-files <ipa-path> <dsym-path>
```
Cross-checks whether all modules in an `.ipa` file are having corresponding `.dSYM` files.
