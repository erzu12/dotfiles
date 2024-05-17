if vim.g.lsp_test == 1 then
    local config = {
        cmd = {'python', '/media/ssd2/dev/HSLU/BAA/ls/lswrap.py', 'jdtls', '-configuration', '/home/jonas/.cache/jdtls/config', '-data', '/home/jonas/.cache/jdtls/workspace'},
        root_dir = vim.fs.dirname(vim.fs.find({'gradlew', '.git', 'mvnw'}, { upward = true })[1]),
    }
    require('jdtls').start_or_attach(config)
else
    local config = {
        cmd = {'jdtls', '-configuration', '/home/jonas/.cache/jdtls/config', '-data', '/home/jonas/.cache/jdtls/workspace'},
        root_dir = vim.fs.dirname(vim.fs.find({'gradlew', '.git', 'mvnw'}, { upward = true })[1]),
    }
    require('jdtls').start_or_attach(config)
end
