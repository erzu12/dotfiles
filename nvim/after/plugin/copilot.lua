--vim.keymap.set("i", "<C-l>", 'copilot#Accept("")', { silent = true, expr = true, replace_keycodes = false })
--vim.keymap.set("i", "<C-k>", "copilot#Previous()", { silent = true, expr = true, replace_keycodes = false })
--vim.keymap.set("i", "<C-j>", "copilot#Next()", { silent = true, expr = true })

if vim.g.lsp_test ~= 1 then
    require('copilot').setup({
        suggestion = {
            enabled = true,
            auto_trigger = true,
            debounce = 75,
            keymap = {
                accept = "<C-l>",
                accept_word = "<C-k>",
                accept_line = "<C-j>",
                next = "<M-k>",
                prev = "<M-j>",
                dismiss = "<C-]>",
            },
        },
    })
end
