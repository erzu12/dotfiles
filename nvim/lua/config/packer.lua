-- This file can be loaded by calling `lua require('plugins')` from your init.vimpack

-- Only required if you have packer configured as `opt`
vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
    -- Packer can manage itself
    use 'wbthomason/packer.nvim'

    use {
        'nvim-telescope/telescope.nvim',
        -- or                            , branch = '0.1.x',
        requires = { { 'nvim-lua/plenary.nvim' } }
    }
    use {
        "nvim-telescope/telescope-file-browser.nvim",
        requires = { "nvim-telescope/telescope.nvim", "nvim-lua/plenary.nvim" }
    }

    --use({
    --    'rose-pine/neovim',
    --    as = 'rose-pine',
    --    config = function()
    --  	  vim.cmd('colorscheme rose-pine')
    --    end
    --})

    use{'zbirenbaum/copilot.lua'}
    use{'stevearc/oil.nvim', config = function() require('oil').setup() end}
    use('nvim-treesitter/nvim-treesitter', { run = ':TSUpdate' })
    use('RRethy/nvim-treesitter-textsubjects')
    use('theprimeagen/harpoon')
    use('mbbill/undotree')
    use('preservim/nerdtree')
    use('tpope/vim-fugitive')
    use('itchyny/lightline.vim')
    use('preservim/nerdcommenter')
    use('windwp/nvim-autopairs')
    use('tpope/vim-surround')
    use('lervag/vimtex')
    use {'kaarmu/typst.vim', ft = {'typst'}}
    use('mfussenegger/nvim-dap')
    use { "rcarriga/nvim-dap-ui", requires = { "mfussenegger/nvim-dap", "nvim-neotest/nvim-nio" } }
    use('jay-babu/mason-nvim-dap.nvim')
    use('ahmedkhalf/project.nvim')
    use('simrat39/symbols-outline.nvim')
    use('christoomey/vim-tmux-navigator')
    use({
        "iamcco/markdown-preview.nvim",
        run = function() vim.fn["mkdp#util#install"]() end,
    })

    use {
        'nvim-tree/nvim-tree.lua',
        requires = {
            'nvim-tree/nvim-web-devicons', -- optional, for file icons
        },
    }
    use {"akinsho/toggleterm.nvim", tag = '*', config = function()
      require("toggleterm").setup()
    end}



    use {
        'VonHeikemen/lsp-zero.nvim',
        requires = {
            -- LSP Support
            { 'neovim/nvim-lspconfig' },
            { 'mfussenegger/nvim-jdtls' },
            { 'williamboman/mason.nvim' },
            { 'williamboman/mason-lspconfig.nvim' },
            { 'https://gitlab.com/schrieveslaach/sonarlint.nvim' },

            -- Autocompletion
            { 'hrsh7th/nvim-cmp' },
            { 'hrsh7th/cmp-buffer' },
            { 'hrsh7th/cmp-path' },
            { 'saadparwaiz1/cmp_luasnip' },
            { 'hrsh7th/cmp-nvim-lsp' },
            { 'hrsh7th/cmp-nvim-lua' },

            -- Snippets
            { 'L3MON4D3/LuaSnip' },
            { 'rafamadriz/friendly-snippets' },
        }
    }

end)
