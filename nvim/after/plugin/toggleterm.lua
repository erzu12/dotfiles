require("toggleterm").setup{
    size = 30,
    open_mapping = [[T]],
    insert_mappings = false, -- whether or not the open mapping applies in insert mode
    direction = 'horizontal',
    close_on_exit = true, -- close the terminal window when the process exits
}

local Terminal  = require('toggleterm.terminal').Terminal
local lazygit = Terminal:new({ cmd = "lazygit", direction = "float", hidden = true })

function lazygit_toggle()
    lazygit:toggle()
end

vim.api.nvim_set_keymap("n", "<leader>g", "<cmd>lua lazygit_toggle()<CR>", {noremap = true, silent = true})


--vim.keymap.set("t", "<Esc>", "<C-\\><C-n><C-w>q" )
