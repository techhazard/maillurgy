Maillurgy
=========

A smtp message parsing library

[![Build Status](https://travis-ci.org/awesomefireduck/maillurgy.svg?branch=master)](https://travis-ci.org/awesomefireduck/maillurgy)
[![Coverage Status](https://coveralls.io/repos/github/awesomefireduck/maillurgy/badge.png?branch=master)](https://coveralls.io/github/awesomefireduck/maillurgy?branch=master)


## Requirements
stable rust && cargo, at least version 1.13

## Contributing
### Fork and clone the repo
Otherwise you are not able to publish your changes 😉

### Setup the testing environment with vagrant:
```sh
# install vagrant plugin to auto install the virtualbox guest additions
# which are needed for the nfs share
$ vagrant plugin install vagrant-vbguest

# boot the testing environment for the first time
# this will take a few minutes...
# all subsequent boots will be faster (a few seconds)
$ vagrant up

# go into the testing environment
$ vagrant ssh

# once inside, run the tests:
vagrant@development:~$ cd ~/smtp
vagrant@development:~/smtp$ cargo test
```

### Install the git hooks
```sh
$ sh scripts/hooks/install.sh
# optional, test if it is working
$ git commit --allow-empty -m 'empty commit' 2>&1 | grep -q 'Compiling maillurgy' && echo "hook installed"; git res
et origin/master
```

### Development
```sh
# get latest changes
$ git pull origin master
# test if everything is working before you add changes
$ cargo test
# check out a feature branch
$ git checkout -b 'new-code-that-makes-stuff-happen'
# do some hacking in your editor of choice
$ nvim src/
```
### Pushing changes
```sh
# commit your changes
$ git add -p
# commit your changes (this will run the hook)
$ git commit
# publish your changes
$ git push --set-upstream origin your-branchname-here
```
Then please make a pull request to awesomefireduck/maillurgy on branch master

## Name
Maillurgy is a forging of the words metallurgy and mail.
