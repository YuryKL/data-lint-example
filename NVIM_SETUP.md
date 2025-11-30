# Neovim Integration for data-lint

## Quick Start

### Option 1: Source the plugin manually

Open Neovim and run:
```vim
:lua dofile(vim.fn.expand("~/Documents/data-lint-example/data-lint.lua")).setup({ lint_on_save = true })
```

Then open an HTML file with pro plugins and save or run:
```vim
:DataLint
```


### Option 2: Use as a proper plugin (not recommended this is an example linter)

Copy the plugin to your Neovim config:

```bash
mkdir -p ~/.config/nvim/lua
cp ~/Documents/data-lint-example/data-lint.lua ~/.config/nvim/lua/
```

Then in your `init.lua`:

```lua
require('data-lint').setup({
  lint_on_save = true,
})
```

## Usage

### Commands

- `:DataLint` - Run the linter on the current file
- `:DataLintClear` - Clear all linter diagnostics

### Keymaps (optional)

Add to your config:

```lua
vim.keymap.set('n', '<leader>dl', '<cmd>DataLint<cr>', { desc = 'Run data-lint' })
vim.keymap.set('n', '<leader>dc', '<cmd>DataLintClear<cr>', { desc = 'Clear data-lint diagnostics' })
```

## Configuration

The plugin supports these options in `setup()`:

- `binary_path` (string): Path to the data-lint binary
- `lint_on_save` (boolean): Auto-lint when saving files
- `lint_on_enter` (boolean): Auto-lint when entering a buffer

## Supported File Types

- `.html`
- `.heex` (Phoenix/Elixir)
- `.templ` (Go templates)
- `.blade.php` (Laravel Blade)

## How It Works

The plugin:
1. Runs the data-lint binary on your file
2. Parses the output
3. Sets Neovim diagnostics that show up in your editor
4. Works with your existing diagnostic configuration (signs, virtual text, etc.)
