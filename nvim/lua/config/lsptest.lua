
local printRedraw = function(msg)
    print(msg)
    vim.api.nvim_command("redraw")
end

local errorRedraw = function(msg)
    vim.api.nvim_err_writeln(msg)
    vim.api.nvim_command("redraw")
end

_G.getFirstFunction = function()
    local result = ""
    local params = {query = ""}
    vim.lsp.buf_request_all(0, "textDocument/documentSymbol", params, function(result)
        for i, res in pairs(result) do
            if res.error then
                errorRedraw("Error when finding workspace symbols: " .. res.error.message)
                return
            end

            for _, item in ipairs(res.result) do
                if item.kind == 12 then
                    errorRedraw("Found function: " .. vim.inspect(item))
                    -- jump to the first function found
                    local offset_encoding = vim.lsp.get_client_by_id(i).offset_encoding
                    vim.lsp.util.jump_to_location(item.location, offset_encoding, false)
                    break
                end
            end
        end
    end)
end


local getClientId = function(client_name)
    for _, client in ipairs(vim.lsp.get_active_clients()) do
        if client.name == client_name then
            return client.id
        end
    end
    return nil
end

-- get the value of a nested table i.e { "a", "b"} -> table["a"]["b"]
local getNestedTableVal = function(table, keys)
    if type(keys) == "string" then
        keys = {keys}
    end
    local val = table
    for _, key in ipairs(keys) do
        val = val[key]
    end
    return val
end


_G.sendLspRequest = function(request, client_id, capabilitie, params)
    if not params then
        params = vim.lsp.util.make_position_params(0)
    end
    if not getNestedTableVal(vim.lsp.get_client_by_id(client_id).server_capabilities, capabilitie) then
        printRedraw("Request: " .. request .. " not supported by server: " .. client_id)
        return
    end

    local result = vim.lsp.buf_request_sync(0, request, params, 60000)
    if not result then
        errorRedraw("Request did not receive result: " .. request)
        return
    end
    local res = result[client_id]
    if res.error then
        errorRedraw(request .. " failed with error: " .. res.error.message)
    else
        printRedraw(request .. " succeeded")
        return res.result
    end
end

local makeTextDocumentParams = function()
    return {textDocument = vim.lsp.util.make_text_document_params()}
end

local sleep = function(ms)
    vim.wait(ms, function() return false end)
end

_G.runLspCommands = function(client_name)

    local client_id = getClientId(client_name)
    if not client_id then
        errorRedraw("Client not found: " .. client_name)
        return
    end

    printRedraw("Running tests")
    -- goto
    sendLspRequest("textDocument/definition", client_id, "definitionProvider")
    -- find
    local params = vim.lsp.util.make_position_params(0)
    params["context"] = {includeDeclaration = true}
    sendLspRequest("textDocument/references", client_id, "referencesProvider", params)
    -- hirarchy
    local item = sendLspRequest("textDocument/prepareCallHierarchy", client_id, "callHierarchyProvider")
    if item then
        params = {item = item[1]}
        sendLspRequest("callHierarchy/incomingCalls", client_id, "callHierarchyProvider", params)
        sendLspRequest("callHierarchy/outgoingCalls", client_id, "callHierarchyProvider", params)
    end
    -- highlight
    sendLspRequest("textDocument/documentHighlight", client_id, "documentHighlightProvider")
    -- links
    local link = sendLspRequest("textDocument/documentLink", client_id, "documentLinkProvider", makeTextDocumentParams())
    if link then
        sendLspRequest("documentLink/resolve", client_id, {"documentLinkProvider", "resolveProvider"}, link[1])
    end
    -- hover
    sendLspRequest("textDocument/hover", client_id, "hoverProvider")
    -- codeLens
    -- ranges
    -- documentSymbol
    sendLspRequest("textDocument/documentSymbol", client_id, "documentSymbolProvider", makeTextDocumentParams())
    -- semanticTokens
    -- inlayHints
    -- moniker
    -- completion
    if client_name == "html" then
        vim.api.nvim_input("o<")
    else
        vim.api.nvim_input("oa")
    end
    -- sleep(1000)
    local compItems = sendLspRequest("textDocument/completion", client_id, "completionProvider")
    if compItems and compItems.items[1] then
        sendLspRequest("completionItem/resolve", client_id, {"completionProvider", "resolveProvider"}, compItems.items[1])
    else
        errorRedraw("No completion items found. Error: " .. vim.inspect(compItems))
    end
    vim.api.nvim_input("<esc>u")
    -- signatureHelp
    sendLspRequest("textDocument/signatureHelp", client_id, "signatureHelpProvider")
    -- formatting
    params = vim.lsp.util.make_formatting_params()
    sendLspRequest("textDocument/formatting", client_id, "documentFormattingProvider", params)
    -- rename
    params = vim.lsp.util.make_position_params(0)
    params["newName"] = "newName"
    sendLspRequest("textDocument/rename", client_id, "renameProvider", params)
    -- linkedEditingRange
    sendLspRequest("textDocument/linkedEditingRange", client_id, "linkedEditingRangeProvider")
end

_G.runCodeActionTests = function(client_name)
    local client_id = getClientId(client_name)
    if not client_id then
        errorRedraw("Client not found: " .. client_name)
        return
    end

    -- codeAction
    local params = vim.lsp.util.make_range_params()
    params["context"] = {diagnostics = vim.diagnostic.get(0)}
    local test = sendLspRequest("textDocument/codeAction", client_id, "codeActionProvider", params)
    errorRedraw("Code action: " .. vim.inspect(test))
end

local waitForLsReady = function(client_name, timeout)
    if not timeout then
        timeout = 10
    end
    if vim.wait(10000, function()
        local client_id = getClientId(client_name)
        return client_id and vim.lsp.get_client_by_id(client_id).server_capabilities
    end) then
        printRedraw("Lsp server ready: " .. client_name)
        return true
    end
    errorRedraw("Lsp server not ready after timeout: " .. client_name)
    return false
end

local waitForLsShutdown = function(timeout)
    if not timeout then
        timeout = 20
    end
    if vim.wait(20000, function()
        return vim.tbl_isempty(vim.lsp.get_active_clients())
    end) then
        printRedraw("Lsp server shutdown")
        return true
    end
    errorRedraw("Lsp server not shutdown after timeout")
    errorRedraw("Active clients: " .. vim.inspect(vim.lsp.get_active_clients()))
    return false
end

local runTestAtLocation = function(working_dir, file, line, character, ls_name)
    printRedraw("Running test for: " .. working_dir)
    vim.lsp.stop_client(vim.lsp.get_active_clients())
    if not (waitForLsShutdown() or waitForLsShutdown()) then
        return false
    end
    vim.api.nvim_set_current_dir(working_dir)
    vim.api.nvim_command("e " .. file)
    vim.api.nvim_command("normal " .. line .. "G" .. character .. "|")

    printRedraw("Waiting for LSP server to be ready")
    if waitForLsReady(ls_name) then
        runLspCommands(ls_name)
    end
    return true
end

_G.runJavaTest = function()
    local testProjectsDir = "/media/ssd2/dev/HSLU/BAA/test-projects/"
    -- java
    if not runTestAtLocation(testProjectsDir .. "spring-framework", "spring-messaging/src/jmh/java/org/springframework/messaging/simp/broker/DefaultSubscriptionRegistryBenchmark.java", 94, 18, "jdtls") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "jlibgamma", "src/libgamma/Partition.java", 55, 16, "jdtls") then
        return
    end
end

_G.runTest = function()
    local testProjectsDir = "/media/ssd2/dev/HSLU/BAA/test-projects/"
    -- c
    if not runTestAtLocation(testProjectsDir .. "postgres", "src/backend/catalog/storage.c", 316, 26, "clangd") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "dmenu", "dmenu.c", 312, 22, "clangd") then
        return
    end
    -- cpp
    if not runTestAtLocation(testProjectsDir .. "inkscape", "src/trace/imagemap.cpp", 62, 14, "clangd") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "MString", "src/MString.cpp", 354, 15, "clangd") then
        return
    end
    -- js
    if not runTestAtLocation(testProjectsDir .. "material-ui", "packages/markdown/loader.js", 510, 26, "tsserver") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "qrious", "dist/qrious.js", 136, 5, "tsserver") then
        return
    end
    -- ts
    if not runTestAtLocation(testProjectsDir .. "vscode", "src/vs/editor/common/model/textModel.ts", 1759, 33, "tsserver") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "vscode-telemetry", "src/producer.ts", 42, 22, "tsserver") then
        return
    end
    -- py
    if not runTestAtLocation(testProjectsDir .. "transformers", "src/transformers/data/processors/utils.py", 147, 19, "pyright") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "tomli", "src/tomli/_re.py", 77, 29, "pyright") then
        return
    end
    -- html
    if not runTestAtLocation(testProjectsDir .. "blink", "LayoutTests/fullscreen/full-screen-table-section.html", 10, 14, "html") then
        return
    end
    if not runTestAtLocation(testProjectsDir .. "bluecloth", "spec/data/markdowntest/Code Blocks.html", 6, 2, "html") then
        return
    end

end

