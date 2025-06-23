#!/usr/bin/env python3
"""
Validation script for the GitHub Actions workflow
"""
import yaml
import sys

def validate_workflow():
    with open('.github/workflows/cross-compile.yml', 'r') as f:
        workflow = yaml.safe_load(f)
    
    # Basic structure validation
    required_keys = ['name', 'jobs']
    for key in required_keys:
        if key not in workflow:
            print(f"❌ Missing required key: {key}")
            return False
    
    # Check for 'on' key (True gets parsed as boolean due to YAML parsing)
    if True not in workflow and 'on' not in workflow:
        print("❌ Missing required key: on")
        return False
    
    jobs = workflow['jobs']
    expected_jobs = ['ci-data', 'cross-compile', 'test-native-gems', 'publish', 'create-release']
    
    for job_name in expected_jobs:
        if job_name not in jobs:
            print(f"❌ Missing expected job: {job_name}")
            return False
        
        job = jobs[job_name]
        if 'runs-on' not in job:
            print(f"❌ Job {job_name} missing 'runs-on'")
            return False
            
        if 'steps' not in job:
            print(f"❌ Job {job_name} missing 'steps'")
            return False
    
    # Check job dependencies
    dependencies = {
        'cross-compile': ['ci-data'],
        'test-native-gems': ['ci-data', 'cross-compile'],
        'publish': ['ci-data', 'cross-compile', 'test-native-gems'],
        'create-release': ['publish']
    }
    
    for job_name, expected_deps in dependencies.items():
        job = jobs[job_name]
        if 'needs' not in job:
            print(f"❌ Job {job_name} missing 'needs' dependency")
            return False
        
        needs = job['needs']
        if isinstance(needs, str):
            needs = [needs]
        
        for dep in expected_deps:
            if dep not in needs:
                print(f"❌ Job {job_name} missing dependency on {dep}")
                return False
    
    # Check for matrix strategy in cross-compile job
    cross_compile = jobs['cross-compile']
    if 'strategy' not in cross_compile:
        print("❌ cross-compile job missing strategy")
        return False
    
    strategy = cross_compile['strategy']
    if 'matrix' not in strategy:
        print("❌ cross-compile job missing matrix strategy")
        return False
    
    # Check for environment protection on publish job
    publish = jobs['publish']
    if 'environment' not in publish:
        print("⚠️  publish job should use environment protection (recommended)")
    
    # Check trigger conditions
    on_config = workflow.get(True, workflow.get('on', {}))
    if 'push' not in on_config or 'tags' not in on_config['push']:
        print("❌ Workflow should trigger on tag pushes")
        return False
    
    if 'workflow_dispatch' not in on_config:
        print("❌ Workflow should support manual dispatch")
        return False
    
    print("✅ Workflow validation passed!")
    return True

if __name__ == '__main__':
    if validate_workflow():
        sys.exit(0)
    else:
        sys.exit(1)