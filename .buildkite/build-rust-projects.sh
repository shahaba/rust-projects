#!/bin/bash

# Start the pipeline file
echo "steps:" > dynamic-pipeline.yml

# Find directories with a Makefile and generate build steps
find . -name 'Cargo.toml' | while read -r makefilePath; do
  dir=$(dirname "$makefilePath")
  echo "  - label: \"Build $(basename "$dir")\"" >> dynamic-pipeline.yml
  echo "    command:" >> dynamic-pipeline.yml
  echo "      - \"cd '$dir' && cargo build && cargo test\"" >> dynamic-pipeline.yml
done

# Upload the dynamic pipeline to Buildkite
buildkite-agent pipeline upload dynamic-pipeline.yml