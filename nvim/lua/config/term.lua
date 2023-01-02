vim.g.termCount = 0
vim.g.currentTerm = 0

function OpenTerminal()
    SetCWD()
    if vim.g.termCount ~= 0 then
        vim.cmd('bo sb Term' .. vim.g.currentTerm)
        vim.cmd(':startinsert')
    else
        vim.cmd(':bo split')
        NewTerminal()
    end
end

function NewTerminal()
    vim.cmd(':term')
    vim.g.termCount = vim.g.termCount + 1
    vim.g.currentTerm  = vim.g.termCount
    vim.cmd('file Term' .. vim.g.termCount)
    vim.cmd(':startinsert')
end

function SwitchTerminal(up)
    if up then
        if vim.g.currentTerm + 1 <= vim.g.termCount then
            vim.g.currentTerm = vim.g.currentTerm + 1
            vim.cmd('b Term' .. vim.g.currentTerm)
        end
    else
        if vim.g.currentTerm - 1 >= 1 then
            vim.g.currentTerm = vim.g.currentTerm - 1
            vim.cmd('b Term' .. vim.g.currentTerm)
        end
    end
    vim.cmd(':startinsert')
end

vim.g.root = ""
function SetCWD()
    vim.cmd('let root = g:NERDTree.ForCurrentTab().getRoot().path.str()')
    vim.cmd('cd' .. vim.g.root)
end

vim.keymap.set("n", "T", function() OpenTerminal() end)
vim.keymap.set("t", "<C-n>", "<C-\\><C-n>:lua NewTerminal()<CR>")
vim.keymap.set("t", "<C-j>", "<C-\\><C-n>:lua SwitchTerminal(false)<CR>" )
vim.keymap.set("t", "<C-k>", "<C-\\><C-n>:lua SwitchTerminal(true)<CR>" )

vim.keymap.set("t", "<Esc>", "<C-\\><C-n><C-w>q" )

local autocmd = vim.api.nvim_create_autocmd
autocmd("TermOpen", {
    pattern = '*',
    command = 'setlocal nonumber'
})
