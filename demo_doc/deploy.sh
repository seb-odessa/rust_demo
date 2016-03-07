#!/bin/bash

rev=$(git rev-parse --short HEAD)
cargo doc 
cd target/doc

git init
git config user.name "seb-odessa"
git config user.email "seb@ukr.net"


#git@github.com:seb-odessa/rust_demo.git

git remote add upstream "git@github.com:seb-odessa/rust_demo"
git fetch upstream && git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
