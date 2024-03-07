
# Contributing

🚀 Thank you for contributing to cdevents! 🚀

## Architecture

The workspace is composed of:

- `cdevents-sdk`: the published rust crate
- `cdevents-specs`: the different version of cdevent's specifications used to generate and validate part of `cdevents-sdk`
- `generator`: the code generator, used to generate a part of `cdevents-sdk` from `cdevents-specs`

Few design's rules:

- The generated code is commited, so users of `cdevents-sdk` don't to generate it at each build.
- TO COMPLETE

## Setting up a development environment

### Setup a GitHub account accessible via SSH

GitHub is used for project Source Code Management (SCM) using the SSH protocol for authentication.

1. Create [a GitHub account](https://github.com/join) if you do not already have one.
1. Setup
[GitHub access via SSH](https://help.github.com/articles/connecting-to-github-with-ssh/)

### Install tools

You must install these tools:

1. [`git`](https://help.github.com/articles/set-up-git/): For source control

2. If you use [mise](https://mise.jdx.dev/): `mise install` (after git clone)
  Else look into the mise configuration file [`.mise.toml`](https://github.com/cdevents/sdk-rust/blob/main/.mise.toml) to have the list of tools to install

### Setup a fork

The sdk-rust project requires that you develop (commit) code changes to branches that belong to a fork of the `cdevents/sdk-rust` repository in your GitHub account before submitting them as Pull Requests (PRs) to the actual project repository.

1. [Create a fork](https://help.github.com/articles/fork-a-repo/) of the `cdevents/sdk-rust` repository in your GitHub account.

1. Create a clone of your fork on your local machine:

    ```shell
    git clone git@github.com:${YOUR_GITHUB_USERNAME}/sdk-rust.git
    ```

1. Configure `git` remote repositories

    Adding `cdevents/sdk-rust` as the `upstream` and your fork as the `origin` remote repositories to your `.git/config` sets you up nicely for regularly [syncing your fork](https://help.github.com/articles/syncing-a-fork/) and submitting pull requests.

    1. Change into the project directory

        ```shell
        cd sdk-rust
        ```

    2. Retrieve submodules

        ```shell
        git submodule init
        git submodule update --init --recursive
        ```

    3. Configure sdk-rust as the `upstream` repository

        ```shell
        git remote add upstream git@github.com:cdevents/sdk-rust.git

        # Optional: Prevent accidental pushing of commits by changing the upstream URL to `no_push`
        git remote set-url --push upstream no_push
        ```

    4. Configure your fork as the `origin` repository

        ```shell
        git remote add origin git@github.com:${YOUR_GITHUB_USERNAME}/sdk-rust.git
        ```

## Development

### Work into a branch

```shell
git switch -c feat_foo
```

### Building and testing

To format the rust code and imports:

```shell
make fmt
```

To run the go linter:

```shell
make lint
```

To run unit tests:

```shell
make test
```

### Commit & push

Commit's message should follow the [conventional commit] convention:
e.g.

```txt
feat: my super feature

...
```

```txt
fix: bug blablabla

FIX #99
```

### Review & merge

Create a PR (pull request) and ask for review.
The last reviewer, will "Squash & merge" when ready.
The message of the squashed commit follows the [conventional commit], and aggregate/summaries commits of the branch.

  [conventional commit]: https://www.conventionalcommits.org/en/v1.0.0/#summary