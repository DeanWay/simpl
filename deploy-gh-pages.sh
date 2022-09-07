#!/bin/bash -e

git checkout master
cd wordle-ui
trunk build
cd ..
git checkout gh-pages
mv wordle-ui/dist/* .
git add --all
git commit -m'update gh-pages'
git push -u origin gh-pages
git checkout master
