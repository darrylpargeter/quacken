# Quacken

Quacken is a small URL expandeder it lets you type the following in the browser
`g/q` and open up at `https://github.com/darrylpargeter/quacken`

it is currently only linux based

adding the following to the config file
```
[g]
expand = "github.com"
```
will open up github if you type `g/` (`/`) is needed or the browser just kind of dies

your can also nest the key to and extra values to the url like so
```
[g.d]
expand = "darrylpargeter" 
```

will open up `github.com/darrylpargeter`, there is no limit to how may levels you can nest

## Setup

### Get nightly rust
`rustup default nightly`

set up nightly to only run in current dir
`rustup override set nightly`

### build project
`cargo build`

### copy binery to location
`sudo cp target/debug/quacken /usr/local/bin/quacken`

### copy service to location
sudo cp quacken@.service /etc/systemd/system/

### copy rocket config to working dir
sudo cp `Rocket.toml` to `/etc/quacken`

### set up config
in `.config` create the dir `quacken`
add the file `config.toml` there is an example in the repo

## Start up
replace username with your username on the computer it is running on
`systemctl start quacken@<username>.service`

## run at boot
`systemctl enable quacken@<username>.service`

## Stop service
`systemctl stop quacken@<username>.service`

## Debug
this will output the logs
`sudo journalctl -u quacken.service -e -f`

## TODO
- [] simpleify set up
- [] get it working for at lest mac
- [] change workingDir to local .config in service
- [] create a small cli to that will do CRUD the `config.toml`
