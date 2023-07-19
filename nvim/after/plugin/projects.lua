require("project_nvim").setup {
    detection_methods = {"lsp", "pattern" },
    patterns = {".git", "_darcs", ".hg", ".bzr", ".svn", "Makefile", "package.json"},
    ignore_lsp = {"sumneko_lua"},
}

require('telescope').load_extension('projects')
vim.keymap.set('n', '<C-p>', ":Telescope projects<CR>", {})

function sleep(n)
  os.execute("sleep " .. tonumber(n))
end


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



