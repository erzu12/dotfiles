local lsp = require("lsp-zero")

lsp.preset("recommended")

lsp.ensure_installed({
    'tsserver',
    'rust_analyzer',
})



local cmp = require('cmp')
local cmp_select = { behavior = cmp.SelectBehavior.Select }
local cmp_mappings = lsp.defaults.cmp_mappings({
    ['<C-y>'] = cmp.mapping.confirm({ select = true }),
    ["<C-Space>"] = cmp.mapping.complete(),
    ["<Tab>"] = cmp.mapping(function(fallback)
        if cmp.visible() then
            cmp.select_next_item()
        else
            fallback()
        end
    end, {
        "i",
        "s",
    }),
    ["<S-Tab>"] = cmp.mapping(function(fallback)
        if cmp.visible() then
            cmp.select_prev_item()
        else
            fallback()
        end
    end, {
        "i",
        "s",
    }),
})

-- disable completion with tab
-- this helps with copilot setup
lsp.setup_nvim_cmp({
    mapping = cmp_mappings,
    preselect = 'none',
    completion = {
        completeopt = 'menu,menuone,noinsert,noselect'
    },
    sources = {
        { name = 'path' },
        { name = 'nvim_lsp', keyword_length = 1 },
        { name = 'buffer', keyword_length = 1 },
        { name = 'luasnip', keyword_length = 2 },
    },
})

lsp.set_preferences({
    suggest_lsp_servers = false,
    set_lsp_keymaps = false,
    sign_icons = {
        error = 'E',
        warn = 'W',
        hint = 'H',
        info = 'I'
    }
})

vim.diagnostic.config({
    virtual_text = true,
})

local function on_list(options)
  vim.fn.setqflist({}, ' ', options)
  vim.api.nvim_command('cfirst')
  require('telescope.builtin').quickfix({initial_mode = "normal"})
end


function set_keys()
    print('set_keys')
    vim.keymap.set("n", "gd", function() vim.lsp.buf.definition() end, opts)
    vim.keymap.set("n", "gt", function() vim.lsp.buf.type_definition() end, opts)
    vim.keymap.set("n", "gi", function() vim.lsp.buf.implementation() end, opts)
    vim.keymap.set("n", "B", function() vim.lsp.buf.hover() end, opts)
    vim.keymap.set("n", "<leader>vws", function() vim.lsp.buf.workspace_symbol() end, opts)
    vim.keymap.set("n", "<leader>w", function() vim.diagnostic.open_float() end, opts)
    vim.keymap.set("n", "[d", function() vim.diagnostic.goto_next() end, opts)
    vim.keymap.set("n", "]d", function() vim.diagnostic.goto_prev() end, opts)
    vim.keymap.set("n", "ca", function() vim.lsp.buf.code_action() end, opts)
    vim.keymap.set("n", "gr", function() vim.lsp.buf.references(nil, {on_list=on_list}) end, opts)
    vim.keymap.set("n", "<leader>r", function() vim.lsp.buf.rename() end, opts)
    vim.keymap.set("i", "<C-h>", function() vim.lsp.buf.signature_help() end, opts)
end

lsp.on_attach(function(client, bufnr)
    local opts = { buffer = bufnr, remap = false }

    if client.name == "eslint" then
        vim.cmd [[ LspStop eslint ]]
        return
    end

    set_keys()

end)

require('lspconfig').gdscript.setup({
    on_attach = function(client, bufnr)
        set_keys()
    end
})

require'lspconfig'.asm_lsp.setup{
    filetypes = {
        "asm", "s", "S"
    }
}

local function get_current_dir_name()
    local current_dir = vim.fn.getcwd()
    local current_dir_name = current_dir:match("^.+/(.+)$")
    return current_dir_name
end

if vim.g.lsp_test == 1 then
    print('starting ls in test mode')
    require('lspconfig').clangd.setup({
        cmd = { "python", "/media/ssd2/dev/HSLU/BAA/ls/lswrap.py", "clangd" },
        -- cmd = { "clangd", "--offset-encoding=utf-16" },
        filetypes = { "c", "cpp", "cxx", "objc", "objcpp" },
    })

    require('lspconfig').jdtls.setup({
        cmd = { "jdtls", "-configuration", "/home/jonas/.cache/jdtls/config", "-data", "/home/jonas/.cache/jdtls/workspace" },
        filetypes = { "nonekdjasklj" },
    })

    require('lspconfig').tsserver.setup({
        cmd = { "python", "/media/ssd2/dev/HSLU/BAA/ls/lswrap.py", "typescript-language-server", "--stdio" },
    })

    require('lspconfig').pyright.setup({
        cmd = { "python", "/media/ssd2/dev/HSLU/BAA/ls/lswrap.py", "pyright-langserver", "--stdio" },
    })

    require('lspconfig').html.setup({
        cmd = { "python", "/media/ssd2/dev/HSLU/BAA/ls/lswrap.py", "vscode-html-language-server", "--stdio" }
    })
else
    require('sonarlint').setup({
       server = {
          cmd = {
             'sonarlint-language-server',
             -- Ensure that sonarlint-language-server uses stdio channel
             '-stdio',
             '-analyzers',
             -- paths to the analyzers you need, using those for python and java in this example
             vim.fn.expand("$MASON/share/sonarlint-analyzers/sonarpython.jar"),
             vim.fn.expand("$MASON/share/sonarlint-analyzers/sonarcfamily.jar"),
             vim.fn.expand("$MASON/share/sonarlint-analyzers/sonarjava.jar"),
             vim.fn.expand("$MASON/share/sonarlint-analyzers/sonarjs.jar"),
          }
       },
       filetypes = {
          -- Tested and working
          'python',
          'cpp',
          'java',
          'typescript',
       }
    })

end

lsp.setup()
