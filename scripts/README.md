# Scripts

This directory contains utility scripts for the project.

## Scripts

### `validate-workflow.py`
Validates the GitHub Actions workflow file for structural correctness and expected jobs.

Usage:
```bash
python3 scripts/validate-workflow.py
```

### `test-gem.rb`
Basic functionality test for the gem to verify it loads and basic operations work.

Usage:
```bash
ruby scripts/test-gem.rb
```

This script is used in the cross-compilation workflow to test that cross-compiled gems work correctly on their target platforms.