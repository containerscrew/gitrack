# gitrack

I have a lot of personal/work repositories in my laptop. Sometimes you jump from change to change and forget to upload the commit. This tool looks for untracked changes in your local folders.

Easy and simple.

# Badges

|              |                                                                                                                                                                                                                                                    |
|--------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Language     | ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)                                                                                                                                             |
| Release      | [![Release](https://img.shields.io/github/release/containerscrew/gitrack)](https://github.com/containerscrew/gitrack/releases/latest)                                                                                                              |
| Code         | ![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/gitrack)                                                                                                                                                             |
| CI - Build   | [![Build](https://github.com/containerscrew/gitrack/actions/workflows/build.yml/badge.svg)](https://github.com/containerscrew/gitrack/actions/workflows/build.yml)                                                                                 |
| CI - Release | [![Build](https://github.com/containerscrew/gitrack/actions/workflows/release.yml/badge.svg)](https://github.com/containerscrew/gitrack/actions/workflows/release.yml)                                                                             |
| CI - Test    | [![Build](https://github.com/containerscrew/gitrack/actions/workflows/test.yml/badge.svg)](https://github.com/containerscrew/gitrack/actions/workflows/test.yml)                                                                                   |
| Meta         | [![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit) [![License - MIT](https://img.shields.io/github/license/containerscrew/gitrack)](/LICENSE) |
| Codecov      | [![Codecov](https://codecov.io/gh/containerscrew/gitrack/graph/badge.svg?token=4AI2U4PX4V)](https://codecov.io/gh/containerscrew/gitrack)                                                                                                          |
| Downloads    | [![Downloads](https://img.shields.io/github/downloads/containerscrew/gitrack/total.svg?logo=github)](https://somsubhra.github.io/github-release-stats/?username=containerscrew&repository=gitrack)                                                 |

# Supported Platforms

| Arch   | ARM64 | AMD64 |
|--------|-------|-------|
| darwin | ✅     | ✅   |
| linux  | ✅     | ✅   |

# Installation

## Install latest version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/gitrack/main/install.sh | sh
```

## Install specific release

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/gitrack/main/install.sh | sh -s -- -v "v0.8.0"
```

## Using cargo

* **[Install Cargo](https://rustup.rs/)**

Open a terminal and run:

```shell
git clone https://github.com/containerscrew/gitrack
cd gitrack
make install
```

# Uninstall

```bash
sudo rm /usr/local/bin/gitrack
```

With cargo:

```shell
make uninstall
```

# Usage

```bash
gitrack --help
```

> By default, it will look for untracked changes in your home folder.

Scanning for untracked changes, specific folder:

```bash
gitrack -p /my/folder/path
```

Summarized output:

```bash
gitrack -p /my/folder/path -s
gitrack -s # remember without -p will scan your home folder
```

Number of workers/threads:

```bash
gitrack -p /home/elliot -w 3
```

Diff file changes:

```bash
gitrack -p /home/elliot/gitrack -d # diff is not compatible with -s (summarized)
```

Exclude directories:

```bash
gitrack -p /home/elliot -s -e "/home/elliot/.cache" -e "/home/elliot/.local"
```

# Threads

> The use of threads is not really necessary in this type of tools, unless you have a very large file/folder system. Adding threads does not mean always better performance. I have included them in order to practice their use. **Max 5 threads, default 3**


# Dev

## pre-commit

```bash
cd gitrack/
pre-commit install
```

## Local container

```bash
cd gitrack/
docker run -it --rm -w /app -h gitrack --name gitrack -v $PWD:/app docker.io/rust:1.80.1-slim-bullseye
```

# Example

Scan personal folder summarized:

![example1](img/example1.png)

Scan specific folder with details:

![example2](img/example2.png)

![example3](img/example3.png)

# TODO

* Implement git commit scan for sensitive data using regex. Just for fun. Like gitleaks does.
* Exclude directories

# Links

* [Using cargo in CI](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)

# License

[License](./LICENSE)
