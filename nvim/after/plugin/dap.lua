local dap_breakpoint = {
    error = {
        text = "🚫",
        texthl = "LspDiagnosticsSignError",
        linehl = "",
        numhl = "",
    },
    rejected = {
        text = "",
        texthl = "LspDiagnosticsSignHint",
        linehl = "",
        numhl = "",
    },
    stopped = {
        text = "⭐️",
        texthl = "LspDiagnosticsSignInformation",
        linehl = "DiagnosticUnderlineInfo",
        numhl = "LspDiagnosticsSignInformation",
    },
}

vim.fn.sign_define("DapBreakpoint", dap_breakpoint.error)
vim.fn.sign_define("DapStopped", dap_breakpoint.stopped)
vim.fn.sign_define("DapBreakpointRejected", dap_breakpoint.rejected)

--require("nvim-dap-virtual-text").setup {
--  commented = true,
--}

local dap, dapui = require "dap", require "dapui"
dapui.setup {} -- use default
dap.listeners.after.event_initialized["dapui_config"] = function()
    dapui.open()
end
dap.listeners.before.event_terminated["dapui_config"] = function()
    dapui.close()
end
dap.listeners.before.event_exited["dapui_config"] = function()
    dapui.close()
end


-- local function keymap(lhs, rhs, desc)
--   vim.keymap.set("n", lhs, rhs, { silent = true, desc = desc })
-- end


vim.keymap.set("n", "<F6>", function() require 'dap'.run_to_cursor() end)
vim.keymap.set("n", "<F8>", function() require 'dapui'.eval(vim.fn.input '[Expression] > ') end)
vim.keymap.set("n", "<F2>", function() require 'dapui'.toggle() end)
vim.keymap.set("n", "<F9>", function() require 'dap'.step_back() end)
vim.keymap.set("n", "<F5>", function() require 'dap'.continue() end)
vim.keymap.set("n", "<F7>", function() require 'dapui'.eval() end)
vim.keymap.set("n", "<Leader>h", function() require 'dap.ui.widgets'.hover() end)
vim.keymap.set("n", "<Leader>e", function() require 'dap.ui.widgets'.preview() end)
vim.keymap.set("n", "<F11>", function() require 'dap'.step_into() end)
vim.keymap.set("n", "<F10>", function() require 'dap'.step_over() end)
vim.keymap.set("n", "<Leader>p", function() require 'dap'.pause.toggle() end)
vim.keymap.set("n", "<Leader>c", function() require 'dap'.close() end)
vim.keymap.set("n", "<Leader>dr", function() require 'dap'.repl.toggle() end)
vim.keymap.set("n", "<Leader>b", function() require 'dap'.toggle_breakpoint() end)
vim.keymap.set("n", "<Leader>q", function() require 'dap'.terminate() end)
vim.keymap.set("n", "<F12>", function() require 'dap'.step_out() end)
