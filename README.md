# 🦀

### an I/O project to mimic the [`ripgrep`](https://github.com/BurntSushi/ripgrep) command line tool which is built in **Rust**.

Rust’s *speed*, *safety*, *single binary output*, and *cross-platform support* make it an ideal language for creating command line tools, so for this project, I’ll make my own version of the classic command line search tool `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint).

along the way I'll make this command line tool use the terminal features that many other command line tools use. I'll read the value of an environment variable to allow the user to configure the behavior of the tool. I’ll also print error messages to the standard error console stream (`stderr`) instead of standard output (`stdout`) so that, for example, the user can redirect successful output to a file while still seeing error messages onscreen.