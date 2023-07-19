require("symbols-outline").setup {
    auto_close = true,
    keymaps = { -- These keymaps can be a string or a table for multiple keys
        hover_symbol = "<C-b>",
    },
}

vim.keymap.set("n", "'", ":SymbolsOutline<CR>")
