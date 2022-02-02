use chrono::Utc;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    /**********************************************************
     *                  Parse cli arguments                   *
     *********************************************************/
    let cli_arguments = cli::get_cli_arguments();

    /**********************************************************
     *                    Get config info                     *
     **********************************************************/
    let lang_config = config::load_language_config(&cli_arguments.language);
    let template_path = config::get_template_path(&cli_arguments.language);
    let filename = format!(
        "{}{}",
        &cli_arguments.filename, &lang_config.file_extension[&cli_arguments.filetype]
    );

    // TODO: get self defined template keywords

    /**********************************************************
     *                     Render header                      *
     **********************************************************/
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_template_file("header", template_path)
        .unwrap();

    let mut vars: HashMap<&str, &str> = HashMap::new();
    // Predefined values
    vars.insert("lc", &lang_config.line_comment);
    let date = Utc::now().format("%d %B %Y").to_string(); // TODO: get format from config
    vars.insert("date", &date);
    vars.insert("fn", &filename);
    vars.insert("lang", &cli_arguments.language);
    // Configured values
    vars.insert("project-name", "Test project"); // TODO: get keywords from config
    vars.insert("project-description", "A project to test sgen");
    vars.insert("organization", "UHasselt/KULeuven - FIIW");
    vars.insert("author", "Lowie Deferme");

    let header = handlebars.render("header", &vars).unwrap();

    // TODO: Write shebang if requested

    /**********************************************************
     *                Write header to new file                *
     **********************************************************/
    let file_path = match cli_arguments.path {
        Some(p) => Path::new(&p).join(&filename),
        None => Path::new(&filename).to_path_buf(),
    };
    println!(
        "Create {:#?} with header:\n-----------------------------\n\n{}",
        file_path, header
    );
    fs::write(file_path, header).expect("Unable to write file");
}

mod cli {
    use clap::Parser;

    #[derive(Parser)]
    #[clap(about, version, author)]
    pub struct Cli {
        /// Name of file to generate
        pub filename: String,

        /// Language of file
        pub language: String,

        /// Set shebang if defined [NOT IMPLEMENTED]
        #[clap(short, long)]
        pub shebang: bool,

        /// Type: source, header, ...
        #[clap(short = 't', long = "type", default_value = "source")]
        pub filetype: String,

        /// Path to create file [current dir if omitted]
        #[clap(short = 'p', long = "path")]
        pub path: Option<String>,
    }

    pub fn get_cli_arguments() -> Cli {
        Cli::parse()
    }
}

mod config {
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    #[derive(Deserialize, Debug, Clone)]
    pub struct LangConfig {
        pub language: String,
        #[serde(rename = "line-comment")]
        pub line_comment: String,
        #[serde(rename = "file-extension")]
        pub file_extension: HashMap<String, String>,
        pub shebang: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct LangConfigAll {
        languages: Vec<LangConfig>,
    }

    pub fn load_language_config(language: &String) -> LangConfig {
        let mut path = env::current_dir()
            .unwrap()
            .join(".hgen")
            .join("languages.json");
        if !path.exists() {
            path = std::path::Path::new(&env::var("XDG_CONFIG_HOME").unwrap())
                .join("hgen")
                .join("languages.json");
        }
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let language_config_all: LangConfigAll = serde_json::from_reader(reader).unwrap();

        let language_config: Vec<LangConfig> = language_config_all
            .languages
            .into_iter()
            .filter(|lang_conf| &lang_conf.language == language)
            .collect();

        return language_config
            .first()
            .expect("Language was not specified")
            .clone();
    }

    /// Get the template path
    ///
    /// Search in the following order:
    ///
    /// - `./.hgen/templates/{language}.hbs`
    /// - `./.hgen/templates/master.hbs`
    /// - `$XDG_CONFIG_HOME/templates/{language}.hbs`
    /// - `$XDG_CONFIG_HOME/templates/master.hbs`
    pub fn get_template_path(language: &String) -> std::path::PathBuf {
        let path = env::current_dir()
            .unwrap()
            .join(".hgen")
            .join("templates")
            .join(format!("{}.hbs", language));
        if path.exists() {
            return path;
        }

        let path = env::current_dir()
            .unwrap()
            .join(".hgen")
            .join("templates")
            .join("master.hbs");
        if path.exists() {
            return path;
        }

        let path = std::path::Path::new(&env::var("XDG_CONFIG_HOME").unwrap())
            .join("hgen")
            .join("templates")
            .join(format!("{}.hbs", language));
        if path.exists() {
            return path;
        }

        let path = std::path::Path::new(&env::var("XDG_CONFIG_HOME").unwrap())
            .join("hgen")
            .join("templates")
            .join("master.hbs");
        return path;
    }
}
