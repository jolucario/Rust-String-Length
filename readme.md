# About this repo 
This code measure the length of the string. Which are EVERYTHING inside "" (except the "" themselves) Yeah that also includes the ``` since this is from a Discord message which are based on track listing of the album The Stanley Parable: Ultra Deluxe – The Complete Soundtrack https://crowscrowscrows.bandcamp.com/album/the-stanley-parable-ultra-deluxe-the-complete-soundtrack

# Note to self: How to publish code to github 
## According to github itself
```
echo "# Rust-String-Length" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/jolucario/Rust-String-Length.git
git push -u origin main
```
## According to myself
```
(add readme.md by whatever means)
git init
git add * ; git config --global user.name "jolucario" ; git config --global user.email "jolucario@protonmail.com" ; git commit -m "first commit"
(remove content inside .gitignore because yes)
git add * ; git config --global user.name "jolucario" ; git config --global user.email "jolucario@protonmail.com" ; git commit -m "second commit"
git branch -M main
git remote add origin https://github.com/jolucario/Rust-String-Length.git
git push -u origin main
```