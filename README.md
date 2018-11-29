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

## Query history
Query a table of last matching jobs:

```
jenq myjobname history
```

Produces:

```
BUILD  UPDATED              RESULT
189   2018-11-27 20:09:44  Some(Success)
188   2018-11-26 17:22:24  Some(Success)
187   2018-11-23 15:33:33  Some(Success)
186   2018-11-22 12:08:15  Some(Success)
185   2018-11-21 17:36:03  Some(Success)
184   2018-11-21 11:40:32  Some(Success)
183   2018-11-21 11:28:07  Some(Success)
```

Note: the build numbers are underlined, **clickable links in your terminal** [if your terminal emulator supports it](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).


## Console output
Get the raw console output of the latest matching job, or numbered job:

```
jenq console myjobname
jenq console myjobname 32
```

Produces the raw console output, here's an excerpt from a [shipcat deploy job](https://github.com/Babylonpartners/shipcat) along with the expected jenkins gunk:

```sh
Started by upstream project "myjobname" build number 32
originally caused by:
 Started by remote host 1.1.1.1
[EnvInject] - Loading node environment variables.
Building remotely on JSG (i-ZZZZZZZZZZZZZ) (generic) in workspace /home/ubuntu/workspace/myjobname
[WS-CLEANUP] Deleting project workspace...
[WS-CLEANUP] Deferred wipeout is used...
[WS-CLEANUP] Done
...
git setup, evar setup, docker setups
...
shipcat::helm::direct: helm --tiller-namespace=dev upgrade webapp charts/base -f webapp.helm.gen.yml --set version=1.0.0
Release "webapp" has been upgraded.
==> v1/Pod(related)
NAME                                     READY  STATUS             RESTARTS  AGE
webapp-6ffbc4f657-6l6bv  0/1    ContainerCreating  0         0s
webapp-75bfb4fb7c-7kdxc  1/1    Running            0         1d

shipcat::kube: Waiting 65s for deployment webapp to rollout (not ready yet)
shipcat::helm::direct: successfully rolled out webapp
Finished: SUCCESS
```


## Latest build

```sh
jenq latest myjobname
```

Produces:

```
myjobnames#189 (478354) at 2018-11-27 20:09:44 UTC on https://jenkins.domain.invalid/job/myjobname/189/
```

## Filtering on string parameters
Query only the entries with a string parameter `APP` whose value is `myapp`. This works on all the subcommands, and can take multiple filters if the jenkins job is parametrised as such:

```
jenq history myjobname -f APP:myapp -f VERSION=0.1.2
```

Last console output for a job with the same parameters:

```
jenq console myjobname -f APP:myapp
```

## Installation
Latest stable with rust installed:

```sh
cargo install jenq # latest stable
```

Latest master with rust installed:

```sh
git clone git@github.com:clux/jenq.git && cd jenq
cargo build
ln -sf $PWD/target/debug/jenq /usr/local/bin/jenq
```

Latest stable without rust installed:

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

## License
Apache 2.0 licensed. See LICENSE for details.

Derivative work from [shipcat](https://github.com/Babylonpartners/shipcat) 0.74.0 licensed under [Apache 2.0](https://github.com/Babylonpartners/shipcat/blob/master/LICENSE)
