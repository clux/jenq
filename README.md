# jenq
[![CircleCI](https://circleci.com/gh/clux/jenq.svg?style=shield)](https://circleci.com/gh/clux/jenq)
[![Crates.io](https://img.shields.io/crates/v/jenq.svg)](https://crates.io/crates/jenq)

Query for jenkins history for parametrised jobs.

## Authentication
Set evars to authenticate with jenkins:

```sh
export JENKINS_URL=https://jenkins.domain.invalid
export JENKINS_API_TOKEN=API_TOKEN_FROM_USER_PAGE
export JENKINS_API_USER=user.name
```

## Usage
Ask `jenq` for `history`, `latest`, or `console` for a named job, optionally specifying filters:

```
jenq history myjobname -f APP:myapp -f VERSION=0.1.2
```

```
jenq console myjobname -f APP:myapp
```

```
jenq latest myjobname
```

**Filters can work in all subcommands**, and match jenkins StringParameters. Here `myjobname` is assumed to have two string parameters: `APP`, and `VERSION`.

The `history` example above gets history where both values of the parameters match.

The `console` example above gets the latest console output where the `APP` parameter matches.

The `latest` example above gets the latest job number from jenkins without matching anything.


## jenq history
Creates a table of last matching jobs (gaps in numbers caused by filters):

```
BUILD  UPDATED              RESULT
189    2018-11-27 20:09:44  Some(Success)
182    2018-11-26 17:22:24  Some(Success)
160    2018-11-23 15:33:33  Some(Success)
130    2018-11-22 12:08:15  Some(Success)
```


Note: the build numbers are underlined, **clickable links in your terminal** [if your terminal emulator supports it](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

## jenq latest
Produces a link and information about last build matching your parameters:

```
myjobnames#189 (478354) at 2018-11-27 20:09:44 UTC on https://jenkins.domain.invalid/job/myjobname/189/
```

## jenq console
Get the raw console output of the latest matching job, or numbered job:

```
jenq console myjobname
jenq console myjobname 32
```

Produces the raw console output, along with the expected jenkins gunk (here truncated):

```sh
Started by upstream project "myjobname" build number 32
originally caused by:
 Started by remote host 1.1.1.1
[EnvInject] - Loading node environment variables.
....🐘.....
shipcat::kube: Waiting 65s for deployment webapp to rollout (not ready yet)
shipcat::helm::direct: successfully rolled out webapp
Finished: SUCCESS
```

## Installation
Latest stable with rust installed:

```sh
cargo install jenq # latest stable
```

Latest stable without rust installed (linux only):

```sh
JENQ_VERSION=0.1.1
curl -sSL https://github.com/clux/jenq/releases/download/${JENQ_VERSION}/jenq.x86_64-unknown-linux-musl.tar.gz | tar xz -C /usr/local
```

Substitute `JENQ_VERSION` variable for the [version you want](https://github.com/clux/jenq/releases).

## Autocompletion
Add this to your `~/.bash_completion` file:

```sh
if hash jenq 2> /dev/null; then
    source <(jenq completions bash)
fi
```

## Development
Clone and build:

```sh
git clone git@github.com:clux/jenq.git && cd jenq
cargo build
ln -sf $PWD/target/debug/jenq /usr/local/bin/jenq
```

## License
Apache 2.0 licensed. See LICENSE for details.

Derivative work from [shipcat](https://github.com/Babylonpartners/shipcat) 0.74.0 licensed under [Apache 2.0](https://github.com/Babylonpartners/shipcat/blob/master/LICENSE)
