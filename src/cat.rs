use std::cell::Cell;
use std::io::{Stderr, StdoutLock};
use std::process::exit;
use std::{fs, io};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CommandLineArgs {
    #[structopt(short = "A", long = "show-all")]
    show_all: bool,
    #[structopt(short = "b", long = "number-nonblank")]
    number_nonblank: bool,
    #[structopt(short = "e")]
    vE: bool,
    #[structopt(short = "E", long = "show-ends")]
    show_ends: bool,
    #[structopt(short = "n", long = "number")]
    number: bool,
    #[structopt(short = "s", long = "squeeze-blank")]
    squeeze_blank: bool,
    #[structopt(short = "t")]
    vT: bool,
    #[structopt(short = "T", long = "show-tabs")]
    show_tabs: bool,
    #[structopt(short = "v", long = "show-nonprinting")]
    show_nonprinting: bool,
    // options以外の引数をpathsとして受け取る
    paths: Vec<String>,
}

struct Program {
    exit_status: Cell<i32>,
    number: bool,
    number_nonblank: bool,
    show_ends: bool,
    show_tabs: bool,
    show_nonprinting: bool,
    squeeze_blank: bool,
    paths: Vec<String>,
}

impl Program {
    fn initialize(stdout: &mut StdoutLock, stderr: &mut Stderr) -> Program {
        let mut cat = Program {
            exit_status: Cell::new(0i32),
            number: false,
            number_nonblank: false,
            show_ends: false,
            show_tabs: false,
            show_nonprinting: false,
            squeeze_blank: false,
            paths: Vec::with_capacity(8),
        };

        let args: CommandLineArgs = CommandLineArgs::from_args();

        if args.show_all {
            println!("{}", "show_all");
            cat.show_nonprinting = true;
            cat.show_ends = true;
            cat.show_ends = true;
        }

        if args.number {
            println!("{}", "number");
            cat.number = true;
            cat.number_nonblank = false;
        }

        if args.number_nonblank {
            println!("{}", "number_nonblank");
            cat.number = true;
            cat.number_nonblank = true;
        }

        if args.show_ends || args.vE {
            println!("{}", "show_ends or vE");
            cat.show_ends = true;
        }

        if args.squeeze_blank {
            println!("{}", "squeeze-blank");
            cat.squeeze_blank = true;
        }

        if args.show_tabs || args.vT {
            println!("{}", "show_tabs or vT");
            cat.show_tabs = true;
        }

        if args.show_nonprinting || args.vE || args.vT {
            println!("{}", "show_nonprinting or vE or vT");
            cat.show_nonprinting = true;
        }

        if !args.paths.is_empty() {
            println!("{:?}", "paths");
            cat.paths = args.paths;
        }
        cat
    }

    fn and_execute(&self, stdout: &mut StdoutLock, stderr: &mut Stderr) -> i32 {
        let stdin = io::stdin();
        let line_count= &mut 0usize;
        let flags_enabled = self.number || self.number_nonblank || self.show_ends || self.show_tabs ||
            self.squeeze_blank || self.show_nonprinting;

        if self.paths.is_empty() && flags_enabled {
            // pathsがなくフラグもない時の処理
            // self.cat(&mut stdin.lock(), line_count, stdout, stderr);
        } else if self.paths.is_empty() {
            // pathsが無い時の処理
            // self.simple_cat(&mut stdin.lock(), stdout, stderr)
        } else {
            for path in &self.paths {
                if flags_enabled && path == "-" {
                    // self.cat(&mut stdin.lock(), line_count, stdout, stderr);
                } else if path == "-" {
                    // self.simple_cat(&mut stdin.lock(), stdout, stderr)
                } else if fs::metadata(&path).map(|m| m.is_dir()).unwrap_or(false) {

                } else if flags_enabled {

                } else {

                }
            }
        }
        self.exit_status.get()
    }
}

fn cat() {}

fn simple_cat() {}

// add_flagはarg-parserの自作メソッド
// https://gitlab.redox-os.org/redox-os/arg-parser/-/blob/master/src/lib.rs#L133-145

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut stderr = io::stderr();
    Program::initialize(&mut stdout, &mut stderr);
    exit(Program::initialize(&mut stdout, &mut stderr).and_execute(&mut stdout, &mut stderr))
}
