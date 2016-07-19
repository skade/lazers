#!/bin/bash

set -e

# Make a new repo for the gh-pages branch
cd doc

crowbook book.crow

git init
# Add, commit and push files
git add --all .
git commit -m "Built documentation"
git checkout -b gh-pages
git remote add origin git@github.com:skade/lazers.git
git push -qf origin gh-pages

# Cleanup
rm -rf .git
