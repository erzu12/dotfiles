require("nvim-tree").setup({
    sync_root_with_cwd = true,
    renderer = {
        group_empty = true,
    },
    respect_buf_cwd = true,
    update_focused_file = {
        enable = true,
        update_root = true
    },
    actions = {
        open_file = {
            quit_on_open = true,
        }
    }
})

--vim.keymap.set('n', ';', function()
    --require("nvim-tree.api").tree.toggle()
--end)
