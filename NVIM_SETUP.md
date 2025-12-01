# Neovim Setup

## Install

```bash
cargo install --path .
```

## lazy.nvim

```lua
{
  dir = "/path/to/data-lint-example",
  name = "data-lint",
  config = function()
    require('data-lint').setup({ lint_on_save = true })
  end,
}
```

## Vanilla Neovim

Copy `lua/data-lint.lua` to `~/.config/nvim/lua/` then add to `init.lua`:

```lua
require('data-lint').setup({ lint_on_save = true })
```

## Commands

- `:DataLint` - Run linter
- `:DataLintClear` - Clear diagnostics

## Config Options

```lua
{
  binary_path = "data-lint",  -- default
  lint_on_save = false,       -- default
  lint_on_enter = false,      -- default
}
```
