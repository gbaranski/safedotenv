# Safedotenv

Safedotenv is tool to make sure that your .env secrets are safe and not commited by accident after testing something.

## Installation

### Arch Linux

Use AUR Helper to install `safedotenv-git`

##### Using [paru](https://github.com/Morganamilo/paru)
```
paru -S safedotenv-git
```

##### Using [yay](https://github.com/Jguer/yay)
```
yay -S safedotenv-git
```

## User Guide

Basic usage, scanning current directory recursively, assuming .env is present at current directory

```
safedotenv
```


Scanning current directory but with .env file somewhere else

```
safedotenv --env-file somedir/.env
```

Scanning specific directory but with .env file somewhere else

```
safedotenv --env-file somedir/.env ~/some/safe/dir
```


Scanning specific directory but with .env file somewhere else, ignoring few variables in .env

```
safedotenv --env-file somedir/.env --ignored-envs REFRESH_TOKEN ACCESS_TOKEN ~/some/safe/dir
```
