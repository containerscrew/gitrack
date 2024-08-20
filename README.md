# gitrack

I have a lot of personal/work repositories in my laptop. Sometimes you jump from change to change and forget to upload the commit. This tool looks for untracked changes in your local folders.

Easy and simple.

# Installation

* **[Install Cargo](https://rustup.rs/)**

Open a terminal and run:

```shell
git clone https://github.com/containerscrew/gitrack
cd gitrack
make install
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

# Example

Scan personal folder summarized:

![example1](img/example1.png)

Scan specific folder with details:

![example2](img/example2.png)

# TODO

* Implement git commit scan for sensitive data using regex. Just for fun. Like gitleaks does.
* Compile the binary for linux/darwin arm64/amd64.
* Support diff files.

# Links

* [Using cargo in CI](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)

# License

[License](./LICENSE)
