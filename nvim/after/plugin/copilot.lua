vim.keymap.set("i", "<C-l>", 'copilot#Accept("")', { silent = true, expr = true, replace_keycodes = false })
vim.keymap.set("i", "<C-k>", "copilot#Previous()", { silent = true, expr = true, replace_keycodes = false })
vim.keymap.set("i", "<C-j>", "copilot#Next()", { silent = true, expr = true })
vim.g.copilot_no_tab_map = true
