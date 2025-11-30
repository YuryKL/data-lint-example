-- Simple Neovim plugin for data-lint
-- Place this in your Neovim config or source it manually

local M = {}

-- Path to the data-lint binary (adjust as needed)
M.binary_path = vim.fn.expand("~") .. "/Documents/data-lint-example/target/release/data-lint"

-- Namespace for diagnostics
local ns = vim.api.nvim_create_namespace("data_lint")

-- Parse linter output
local function parse_output(output)
  local diagnostics = {}

  for line in output:gmatch("[^\r\n]+") do
    -- Parse format: "file:line:col: data-plugin - message"
    local file, lnum, col, plugin, msg = line:match("([^:]+):(%d+):(%d+): data%-([^ ]+) %- (.+)")

    if lnum and col then
      table.insert(diagnostics, {
        lnum = tonumber(lnum) - 1,  -- 0-indexed
        col = tonumber(col) - 1,     -- 0-indexed
        message = string.format("data-%s - %s", plugin, msg),
        severity = vim.diagnostic.severity.WARN,
        source = "data-lint",
      })
    end
  end

  return diagnostics
end

-- Run linter on current buffer
function M.lint()
  local bufnr = vim.api.nvim_get_current_buf()
  local filename = vim.api.nvim_buf_get_name(bufnr)

  if filename == "" then
    vim.notify("No file associated with buffer", vim.log.levels.WARN)
    return
  end

  -- Check if file is HTML/template
  if filename:match("%.blade%.php$") then
    -- Blade file
  elseif not vim.tbl_contains({"html", "heex", "templ"}, vim.fn.fnamemodify(filename, ":e")) then
    return
  end

  -- Run the linter
  local output = vim.fn.system({M.binary_path, filename})

  -- Parse and set diagnostics
  local diagnostics = parse_output(output)
  vim.diagnostic.set(ns, bufnr, diagnostics, {})

  if #diagnostics > 0 then
    vim.notify(string.format("data-lint: found %d warning(s)", #diagnostics), vim.log.levels.INFO)
  end
end

-- Clear diagnostics
function M.clear()
  local bufnr = vim.api.nvim_get_current_buf()
  vim.diagnostic.reset(ns, bufnr)
end

-- Setup function
function M.setup(opts)
  opts = opts or {}

  if opts.binary_path then
    M.binary_path = vim.fn.expand(opts.binary_path)
  end

  -- Create user commands
  vim.api.nvim_create_user_command("DataLint", M.lint, {})
  vim.api.nvim_create_user_command("DataLintClear", M.clear, {})

  -- Auto-lint on save (optional)
  if opts.lint_on_save then
    vim.api.nvim_create_autocmd("BufWritePost", {
      pattern = {"*.html", "*.heex", "*.templ", "*.blade.php"},
      callback = M.lint,
    })
  end

  -- Auto-lint on buffer enter (optional)
  if opts.lint_on_enter then
    vim.api.nvim_create_autocmd("BufEnter", {
      pattern = {"*.html", "*.heex", "*.templ", "*.blade.php"},
      callback = M.lint,
    })
  end
end

return M
