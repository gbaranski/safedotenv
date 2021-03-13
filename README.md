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


Scanning specific directory but with .env file somewhere else, ignoring `REFRESH_TOKEN` and `ACCESS_TOKEN` variables from .env

```
safedotenv --env-file somedir/.env --ignore-env REFRESH_TOKEN ACCESS_TOKEN ~/some/safe/dir
```

#### Using with git hooks

1. Open `.git/hooks/pre-commit` file(create if does not exits)
2. Add this code
```
#!/bin/sh

safedotenv --silent $(git rev-parse --show-toplevel)
```
3. Done
