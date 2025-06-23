# Cross-Compilation Workflow

This repository includes a GitHub Actions workflow for cross-compiling the `micro_mcp` Ruby gem with Rust extensions for multiple platforms and Ruby versions.

## Features

- **Cross-platform compilation**: Automatically builds gems for Linux (x86_64), macOS (x86_64 and ARM64), and Windows (x64)
- **Multi-Ruby support**: Tests and builds for multiple stable Ruby versions (3.1, 3.2, 3.3)
- **Automated testing**: Tests cross-compiled gems on their target platforms
- **RubyGems publishing**: Automatically publishes to RubyGems when properly configured
- **GitHub releases**: Creates GitHub releases with compiled gems as artifacts

## Workflow Structure

The workflow consists of several jobs:

1. **ci-data**: Fetches supported Ruby platforms and versions dynamically
2. **cross-compile**: Cross-compiles the gem for all supported platforms
3. **test-native-gems**: Tests the cross-compiled gems on their respective platforms
4. **publish**: Publishes gems to RubyGems (on tags or manual release)
5. **create-release**: Creates a GitHub release with gem artifacts

## Configuration

### RubyGems Publishing

To enable automatic publishing to RubyGems, you need to:

1. Create a RubyGems API key at https://rubygems.org/profile/api_keys
2. Add the API key as a repository secret named `RUBYGEMS_AUTH_TOKEN`
3. Set up a production environment in your repository settings (optional but recommended)

### Triggering the Workflow

The workflow can be triggered in two ways:

1. **Automatic on tags**: Push a version tag (e.g., `v1.0.0`) to trigger compilation and publishing
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Manual dispatch**: Use the GitHub Actions UI to manually trigger the workflow with optional release flag

### Customizing Platforms and Ruby Versions

The workflow uses `oxidize-rb/actions/fetch-ci-data` to dynamically determine supported platforms and Ruby versions. You can customize this by modifying the `ci-data` job in `.github/workflows/cross-compile.yml`:

```yaml
- uses: oxidize-rb/actions/fetch-ci-data@v1
  id: fetch
  with:
    supported-ruby-platforms: |
      exclude: [arm-linux]  # Exclude specific platforms
    stable-ruby-versions: |
      exclude: [head]       # Exclude ruby-head builds
```

### Adding New Platforms

To add support for new platforms, ensure they are:
1. Supported by `rb-sys-dock` (the cross-compilation tool)
2. Available in the `oxidize-rb/actions/fetch-ci-data` action
3. Have corresponding test runners in the `test-native-gems` job

## Dependencies

The workflow uses several third-party actions:

- `oxidize-rb/actions/fetch-ci-data@v1`: Fetches supported platforms and Ruby versions
- `oxidize-rb/actions/cross-gem@v1`: Cross-compiles Ruby gems with Rust extensions
- `ruby/setup-ruby@v1`: Sets up Ruby environments
- `actions/upload-artifact@v4` and `actions/download-artifact@v4`: Manages build artifacts
- `softprops/action-gh-release@v2`: Creates GitHub releases

## Troubleshooting

### Cross-compilation Failures

If cross-compilation fails:
1. Check that your Rust extension builds correctly locally
2. Ensure all dependencies are compatible with cross-compilation
3. Review the `rb-sys` and `rb-sys-dock` documentation

### Test Failures

If tests fail on specific platforms:
1. Check platform-specific dependencies in your gemspec
2. Ensure your Rust code handles platform differences correctly
3. Consider adding platform-specific test exclusions if needed

### Publishing Issues

If publishing to RubyGems fails:
1. Verify your `RUBYGEMS_AUTH_TOKEN` secret is set correctly
2. Ensure your API key has the necessary permissions
3. Check that your gem version hasn't already been published

## Extending the Workflow

The workflow is designed to be modular and extensible:

- **Add new test jobs**: Create additional jobs that depend on `cross-compile`
- **Custom build steps**: Add pre/post-build steps in the cross-compilation job
- **Different triggers**: Modify the `on:` section to change when the workflow runs
- **Additional platforms**: Update the test matrix to include new platforms as they become available

For more information about the underlying tools, see:
- [oxidize-rb/actions](https://github.com/oxidize-rb/actions)
- [rb-sys documentation](https://github.com/oxidize-rb/rb-sys)