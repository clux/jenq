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

```sh
jenq myjobname history
```

## Console output
Get the raw console output of the latest matching job, or numbered job:

```sh
jenq myjobname console
jenq myjobname console 32
```

## Latest build

```sh
jenq myjobname latest
```

## Filtering on string parameters
Query only the entries with a string parameter `APP` whose value is `myapp`.

```sh
jenq -f APP:myapp myjobname history
```

Last console output for a job with the same parameters:

```sh
jenq -f APP:myapp myjobname console
```

## License
Apache 2.0 licensed. See LICENSE for details.

Derivative work from [shipcat](https://github.com/Babylonpartners/shipcat) licensed under [Apache 2.0](https://github.com/Babylonpartners/shipcat/blob/master/LICENSE)
