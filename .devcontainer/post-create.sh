#!/bin/bash
set -e

# Take ownership of mounted volumes
sudo chown vscode node_modules
sudo chown vscode ui/node_modules
sudo chown vscode ui/build