use std::io;

mod lexer;

pub fn copyright() -> String {
    let os_info = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "android") {
        "android"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else {
        "unknown"
    };
    let arch_info = if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "powerpc") {
        "powerpc"
    } else if cfg!(target_arch = "powerpc64") {
        "powerpc64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "unknown"
    };
    let env_info = if cfg!(target_env = "gnu") {
        "gnu"
    } else if cfg!(target_env = "msvc") {
        "msvc"
    } else if cfg!(target_env = "musl") {
        "musl"
    } else {
        "unknown"
    };

    "
     __      __       _   _     |  Repo:    github.com/maltlang/rmalt
    |  \\    /  |___ _| |_| |_   |  Version: Malt(rmalt v0.1 Beta), std(lyzhstd null), repl(malt-repl null)
    | \\ \\  / / /  _` | |_   _|  |  License: MIT
    | |\\ \\/ /| | |_| | | | |__  |  Author:  lyzh(Zhihang-liu) github.com/Zhihang-Liu
    |_| \\__/ |_\\___._|_| |___/  |  Target:  ".to_string() + arch_info + "-" + os_info + "-" + env_info + "\n\n" +
        "help-list: use = type
\tuse (help?) get docs (unavailable).
\tuse (exit!) or try use Ctrl-C quit repl.
"
}


fn main() {
    //println!("{}", copyright());
    loop {
        println!(">>> ");
        let mut src = String::new();
        let _ = io::stdin().read_line(&mut src);
        let token_stream = lexer::lexer(src.trim_right_matches("\r")).unwrap();
        println!("{:?}", token_stream);
    }
}
