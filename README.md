# Secure-Cypher

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/) [![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/Naereen/StrapDown.js/graphs/commit-activity) [![Documentation Status](https://readthedocs.org/projects/ansicolortags/badge/?version=latest)](http://ansicolortags.readthedocs.io/?badge=latest)

A self hosted free and open source password manager for linux, Built using Rust, Egui, MongoDB 

Many times we have forgotten our account password and had to recent it which is a hassle and annoying , there are alternative to save password using third party apps but we can never be sure wheather its safe or not.

By using secure cypher you can host your own database and store your passwords in your very own DB and make sure nobody can snoop them and its always secure 

## How it works

The app is connected to MongoDB database , So when the password is typed into the password field and clicked on save, the password get securely saved in your own database, and the user can also click on "show password" to view all your saved password retrived from the database

## Requirments
- Rust
- MongoDB account

## Instructions
Git clone the repo
```
git clone https://github.com/PrethamMuthappa/secure-cipher.git
```

create a .env file in the root of your folder 
```
URL=" PUT YOUR MONGODB CREDENTIALS HERE"
```


Run from your terminal in your current working project directory
```
cargo run

```
   



