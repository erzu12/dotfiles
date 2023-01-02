vim.g.NERDTreeQuitOnOpen = 1
vim.g.NERDTreeShowBookmarks = 1
vim.g.NerdTreeOpen = 0

vim.keymap.set('n', ';', function ()
    vim.cmd("let g:NerdTreeOpen = g:NERDTree.IsOpen()")
    if vim.g.NERDTree and vim.g.NerdTreeOpen == 1 then
        vim.cmd('NERDTreeToggle')
    else
        if vim.fn.getreg('%') == "" then
            vim.cmd('NERDTreeToggle')
        else
            vim.cmd('NERDTreeFind')
        end
    end

end)

