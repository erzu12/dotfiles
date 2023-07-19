vim.g.mapleader = " "

vim.keymap.set("i", "jj", "<esc>")


vim.keymap.set("", "<C-j>", "10j")
vim.keymap.set("", "<C-k>", "10k")

vim.keymap.set("n", "<C-_>", ":noh<CR>")
vim.keymap.set("n", "<C-a>", "$i")

vim.keymap.set("i", "<A-CR>", "<esc>O")

vim.keymap.set("v", "<S-j>", ":m '>+1<CR>gv=gv")
vim.keymap.set("v", "<S-k>", ":m '<-2<CR>gv=gv")


vim.keymap.set("n", "S" ,"<C-w>v")
vim.keymap.set("n", "E" ,"<C-w>s")
vim.keymap.set("n", "J" ,"<C-w><down>")
vim.keymap.set("n", "K" ,"<C-w><up>")
vim.keymap.set("n", "H" ,"<C-w><left>")
vim.keymap.set("n", "L" ,"<C-w><right>")
vim.keymap.set("n", "Q" ,"<C-w>q")

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
vim.keymap.set("n", "<Tab>", ":ClangdSwitchSourceHeader<CR>")

vim.keymap.set("n", "<C-b>", ":TermExec cmd='./run.sh'<CR><C-w><down>")

