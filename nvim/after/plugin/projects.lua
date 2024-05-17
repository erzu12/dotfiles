require("project_nvim").setup {
    detection_methods = {"lsp", "pattern" },
    patterns = {".git", ".svn", "Makefile", "package.json", "run.sh", "cargo.toml"},
    ignore_lsp = {"lua_ls"},
}

require('telescope').load_extension('projects')
vim.keymap.set('n', '<C-p>', ":Telescope projects<CR>", {})


function showProject()
    print(vim.fn.getcwd())
    if vim.fn.getcwd() == "/home/jonas" then
        --vim.cmd(':Telescope projects')
    else
        require('telescope.builtin').oldfiles({ cwd_only = true, initial_mode = "normal"})
    end
end

vim.api.nvim_create_autocmd("VimEnter", {
    pattern = "*",
    callback = function () showProject() end,
})



