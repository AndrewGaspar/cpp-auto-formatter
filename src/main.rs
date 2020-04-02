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
use reqwest::{blocking::Client, header};
use serde::de::DeserializeOwned;

mod github;

use github::*;

fn main() -> Result<(), Box<dyn Error>> {
    App::run()
}

#[derive(Debug)]
struct App {
    clang_format_path: PathBuf,
    includes: Vec<Pattern>,
    excludes: Vec<Pattern>,
    github_workspace: PathBuf,
    bot_name: String,
    github_token: String,
}

impl App {
    fn run() -> Result<(), Box<dyn Error>> {
        let matches = clap::App::new("cpp-auto-format")
            .author("Andrew Gaspar <andrew.gaspar@outlook.com>")
            .about("Runner code for executing clang-format")
            .arg(
                Arg::with_name("github-token")
                    .long("github-token")
                    .takes_value(true)
                    .required(true)
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
            .arg(
                Arg::with_name("bot-name")
                    .long("bot-name")
                    .takes_value(true)
                    .default_value("cpp-auto-formatter")
            )
            .subcommand(SubCommand::with_name("command"))
            .subcommand(SubCommand::with_name("check"))
            .subcommand(SubCommand::with_name("list"))
            .get_matches();

        let clang_format_version = matches.value_of("clang-format-version").unwrap();

        let clang_format_path: PathBuf =
            if let Some(clang_format_override) = matches.value_of("clang-format-override") {
                clang_format_override.into()
            } else if env::var("GITHUB_ACTION").is_ok() {
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

        if let Ok(github_workspace) = env::var("GITHUB_WORKSPACE") {
            env::set_current_dir(PathBuf::from(github_workspace))?;
        }

        let github_token = matches.value_of("github-token").unwrap().to_owned();

        let app = App {
            clang_format_path,
            includes,
            excludes,
            github_workspace: PathBuf::new(),
            bot_name: matches.value_of("bot-name").unwrap().into(),
            github_token,
        };

        if let Some(matches) = matches.subcommand_matches("list") {
            app.list(matches);
            process::exit(0);
        }

        match matches.subcommand() {
            ("command", matches) => app.command(matches.unwrap())?,
            ("check", matches) => app.check(matches.unwrap())?,
            _ => panic!("Unexcepted subcommand"),
        }

        Ok(())
    }

    fn clone(&self, full_name: &str, branch: &str) -> Result<(), Box<dyn Error>> {
        assert!(Command::new("git")
            .args(&[
                "clone",
                "-b",
                branch,
                "--depth",
                "1",
                &format!(
                    "https://x-access-token:{}@github.com/{}.git",
                    self.github_token, full_name
                ),
                ".",
            ])
            .spawn()?
            .wait()?
            .success());

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

    fn output_help(&self, _app: clap::App) {}

    fn configure(&self) -> Result<(), Box<dyn Error>> {
        assert!(Command::new("git")
            .args(&[
                "config",
                "--global",
                "user.email",
                &format!("{}@automation.bot", self.bot_name),
            ])
            .spawn()?
            .wait()?
            .success());

        assert!(Command::new("git")
            .args(&["config", "--global", "user.name", &self.bot_name])
            .spawn()?
            .wait()?
            .success());

        Ok(())
    }

    fn github_client(&self) -> Result<Client, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {}", self.github_token))?,
        );

        Ok(Client::builder().default_headers(headers).build()?)
    }

    fn command(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if env::var("GITHUB_EVENT_NAME")? != "issue_comment" {
            eprintln!("Error: This action is only compatible with 'issue_comment' events");
            process::exit(1);
        }

        let client = self.github_client()?;
        let payload: GitHubIssueCommentEvent = dbg!(load_payload()?);

        if !payload
            .comment
            .body
            .starts_with(&format!("@{}", self.bot_name))
        {
            eprintln!("Error: The command must start with @{}", self.bot_name);
            std::process::exit(1);
        }

        let pull_request = match payload.issue.pull_request {
            Some(pr) => dbg!(dbg!(client.get(&pr.url).send()?).json::<GitHubPullRequest>()?),
            None => {
                eprintln!("Error: cpp-auto-formatter only works with PR comments");
                std::process::exit(1);
            }
        };

        let command_arr = shell_words::split(&payload.comment.body)?;

        let mut app = clap::App::new(&format!("@{}", self.bot_name)).subcommand(
            SubCommand::with_name("format").arg(Arg::with_name("squash").long("squash")),
        );

        let matches = match app.get_matches_from_safe_borrow(command_arr) {
            Ok(matches) => matches,
            Err(_) => {
                self.output_help(app);
                std::process::exit(1);
            }
        };

        let _matches = if let Some(matches) = matches.subcommand_matches("format") {
            matches
        } else {
            self.output_help(app);
            std::process::exit(1);
        };

        let branch = ref_to_branch(&pull_request.head.r#ref);
        self.clone(&pull_request.head.repo.full_name, branch)?;
        self.configure()?;
        self.format_all();

        assert!(Command::new("git")
            .args(&["commit", "-am", "cpp-auto-formatter"])
            .spawn()?
            .wait()?
            .success());

        assert!(Command::new("git")
            .args(&["push"])
            .spawn()?
            .wait()?
            .success());

        Ok(())
    }

    fn check(&self, _matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let payload: GitHubPushEvent = dbg!(load_payload()?);
        let branch = ref_to_branch(&payload.r#ref);
        self.clone(&payload.repository.full_name, branch)?;
        self.format_all();

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

fn load_payload<T: DeserializeOwned>() -> Result<T, Box<dyn Error>> {
    let github_event_path = env::var("GITHUB_EVENT_PATH")?;
    let github_event = std::fs::read_to_string(&github_event_path)?;
    Ok(serde_json::from_str(dbg!(&github_event))?)
}

fn ref_to_branch(r#ref: &str) -> &str {
    let expected_prefix = "refs/heads/";
    assert!(
        r#ref.starts_with(expected_prefix),
        "Unexpected push ref: {}",
        r#ref
    );
    &r#ref[expected_prefix.len()..]
}
