// Preludes
use rayon::prelude::*;

// Standard Imports
use std::{
    env,
    error::Error,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{self, Command, Stdio},
};

// Third Party Imports
use clap::{Arg, ArgMatches, SubCommand};
use glob::Pattern;

fn main() -> Result<(), Box<dyn Error>> {
    App::run()
}

#[derive(Debug)]
struct App {
    clang_format_path: PathBuf,
    includes: Vec<Pattern>,
    excludes: Vec<Pattern>,
    github_workspace: PathBuf,
}

impl App {
    fn run() -> Result<(), Box<dyn Error>> {
        let matches = clap::App::new("cpp-auto-format")
            .author("Andrew Gaspar <andrew.gaspar@outlook.com>")
            .about("Runner code for executing clang-format")
            .arg(
                Arg::with_name("github-action")
                    .long("github-action")
            )
            .arg(
                Arg::with_name("clang-format-version")
                    .long("clang-format-version")
                    .takes_value(true)
                    .default_value("10")
                    .conflicts_with("clang-format-override"),
            )
            .arg(
                Arg::with_name("clang-format-override")
                    .long("clang-format-override")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("include")
                    .long("include")
                    .takes_value(true)
                    .required(true)
                    .value_delimiter(",")
                    .default_value("**/*.c,**/*.h,**/*.C,**/*.H,**/*.cpp,**/*.hpp,**/*.cxx,**/*.hxx,**/*.c++,**/*.h++,**/*.cc,**/*.hh"),
            )
            .arg(
                Arg::with_name("exclude")
                    .long("exclude")
                    .takes_value(true)
                    .value_delimiter(",")
                    .default_value(""),
            )
            .subcommand(SubCommand::with_name("commit"))
            .subcommand(SubCommand::with_name("check"))
            .subcommand(SubCommand::with_name("list"))
            .get_matches();

        let clang_format_version = matches.value_of("clang-format-version").unwrap();

        let clang_format_path: PathBuf =
            if let Some(clang_format_override) = matches.value_of("clang-format-override") {
                clang_format_override.into()
            } else if matches.is_present("github-action") {
                format!("/clang-format/clang-format-{}", clang_format_version).into()
            } else {
                String::from_utf8(
                    Command::new("which")
                        .arg("clang-format")
                        .output()
                        .unwrap()
                        .stdout,
                )
                .unwrap()
                .lines()
                .next()
                .unwrap()
                .into()
            };

        if !clang_format_path.exists() {
            eprintln!("Error: No clang-format version {}", clang_format_version);
            std::process::exit(1);
        }

        let includes = matches
            .values_of("include")
            .unwrap()
            .map(Pattern::new)
            .collect::<Result<Vec<_>, _>>()?;
        let excludes = matches
            .values_of("exclude")
            .unwrap()
            .map(Pattern::new)
            .collect::<Result<Vec<_>, _>>()?;

        if matches.is_present("github-action") {
            env::set_current_dir(PathBuf::from(env::var("GITHUB_WORKSPACE")?))?;
        }

        let app = App {
            clang_format_path,
            includes,
            excludes,
            github_workspace: PathBuf::new(),
        };

        if let Some(matches) = matches.subcommand_matches("list") {
            app.list(matches);
            process::exit(0);
        }

        app.format_all();

        match matches.subcommand() {
            ("commit", matches) => app.commit(matches.unwrap())?,
            ("check", matches) => app.check(matches.unwrap())?,
            _ => panic!("Unexcepted subcommand"),
        }

        Ok(())
    }

    fn list_files<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        BufReader::new(
            Command::new("git")
                .args(&["ls-tree", "-r", "HEAD", "--name-only", "--full-tree"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap(),
        )
        .lines()
        .map(|s| s.unwrap())
        .filter(move |s| self.includes.iter().any(|p| p.matches(&s)))
        .filter(move |s| !self.excludes.iter().any(|p| p.matches(&s)))
    }

    fn format_all(&self) {
        self.list_files().par_bridge().for_each(|file| {
            Command::new(&self.clang_format_path)
                .args(&["-i", &file])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        });
    }

    fn commit(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        process::exit(
            Command::new("git")
                .args(&["commit", "-am", "GitHub clang-format Action"])
                .spawn()?
                .wait()?
                .code()
                .unwrap_or(1),
        );
    }

    fn check(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        process::exit(
            Command::new("git")
                .args(&["diff", "--exit-code"])
                .spawn()?
                .wait()?
                .code()
                .unwrap_or(1),
        )
    }

    fn list(&self, _matches: &ArgMatches) {
        for file in self.list_files() {
            println!("{}", file);
        }
        process::exit(0)
    }
}
