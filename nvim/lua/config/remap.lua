vim.keymap.set("i", "jj", "<esc>")

vim.keymap.set("v", "<C-j>", ":m '>+1<CR>gv=gv")
vim.keymap.set("v", "<C-k>", ":m '<-2<CR>gv=gv")

vim.keymap.set("", "J", "10j")
vim.keymap.set("", "K", "10k")

vim.keymap.set("n", "<C-_>", ":noh<CR>")
vim.keymap.set("n", "<C-a>", "$i")

vim.keymap.set("i", "<A-CR>", "<esc>O")


vim.keymap.set("n", "S" ,"<C-w>v")
vim.keymap.set("n", "E" ,"<C-w>s")
vim.keymap.set("n", "<C-q>" ,"<C-w>q")


vim.keymap.set("n", "<C-d>", "<C-d>zz")
vim.keymap.set("n", "<C-u>", "<C-u>zz")
vim.keymap.set("n", "n", "nzzzv")
vim.keymap.set("n", "N", "Nzzzv")

vim.keymap.set("n", "p", "p`[=`]")


-- greatest remap ever
vim.keymap.set("x", "<leader>p", "\"_dP")

-- next greatest remap ever : asbjornHaland
vim.keymap.set("n", "<leader>y", "\"+y")
vim.keymap.set("v", "<leader>y", "\"+y")
vim.keymap.set("n", "<leader>Y", "\"+Y")
vim.keymap.set("n", "<leader>d", "\"_d")

vim.keymap.set("v", "<leader>d", "\"_d")

vim.keymap.set("n", "<leader>f", function()
    vim.lsp.buf.format()
end)

vim.keymap.set("n", "<leader>s", ":%s/\\<<C-r><C-w>\\>/<C-r><C-w>/gI<Left><Left><Left>")
vim.keymap.set("n", "<leader>x", "<cmd>!chmod +x %<CR>", { silent = true })

vim.keymap.set("n", "<M-o>", "<C-i>")
vim.keymap.set("n", "<leader><Tab>", ":ClangdSwitchSourceHeader<CR>")

--vim.keymap.set("n", "<C-b>", ":TermExec cmd='./run.sh'<CR><C-w><down>")
-- vim.keymap.set("n", "<C-b>", ':silent ! kitty @send-text --to=unix:/tmp/mykitty "./run.sh\\r"<CR>')
-- vim.keymap.set("n", "<leader>t", ":silent ! kitty --detach --listen-on=unix:/tmp/mykitty<CR>")

vim.keymap.set("n", "T", function()
    os.execute("tmux resize-pane -Z")
    vim.g.auto_zoom = not vim.g.auto_zoom
end)
