## [Unreleased]

### Added
- Ruby `ToolRegistry` for registering tools dynamically
- Access to the MCP runtime from Ruby tools
- Argument support for tools
- Exposed `client_supports_sampling` on runtime
- Exposed `create_message` on runtime
- Prompt registry for registering prompts with arguments and runtime access

### Changed
- Gem renamed from `mcp_lite` to `micro_mcp`
- Tool handling uses a dynamic registry
- Improved server error handling
- Updated dependency `rust-mcp-sdk` to 0.5.0
- Rooted stored Ruby `Proc` objects to avoid GC issues

## [0.1.0] - 2025-06-17

- Initial release
