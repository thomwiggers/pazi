use crate::harness::autojumpers::Autojumper;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

pub struct ShellCmd<'a> {
    pub cmd: &'a str,
    pub env: Vec<(&'a str, String)>,
}

impl Shell {
    pub fn name(&self) -> &'static str {
        match self {
            &Shell::Bash => "bash",
            &Shell::Zsh => "zsh",
            &Shell::Fish => "fish",
        }
    }

    pub fn setup(&self, root: &Path, autojump: &Autojumper, ps1: &str, preinit: &str) {
        let rc_file = match *self {
            Shell::Bash | Shell::Zsh => root.join(format!("home/pazi/.{}rc", self.name())),
            Shell::Fish => root.join(format!("home/pazi/.config/fish/config.fish")),
        };

        let rc_template = match *self {
            Shell::Bash => format!(
                r#"#Autogenerated by pazi integ tests
set -e
{preinit}
export PS1="{ps1}" # sep so we know when our commands finished
export PATH=$PATH:$(dirname "{bin_path}")
{init}
"#,
                bin_path = autojump.bin_path(),
                init = autojump.init_for(self),
                ps1 = ps1,
                preinit = preinit,
            ),
            Shell::Zsh => format!(
                r#"#Autogenerated by pazi integ tests
set -e
unsetopt zle
{preinit}
export PS1="{ps1}" # sep so we know when our commands finished
export PATH=$PATH:$(dirname "{bin_path}")
{init}
"#,
                bin_path = autojump.bin_path(),
                init = autojump.init_for(self),
                ps1 = ps1,
                preinit = preinit,
            ),
            Shell::Fish => format!(
                r#"#Autogenerated by pazi integ tests
# TODO: no 'set -e' equiv https://github.com/fish-shell/fish-shell/issues/510
{preinit}
set fish_greeting
function fish_prompt
    echo -n "{ps1}" # sep so we know when our commands finished
end
set PATH $PATH (dirname {bin_path})
{init}
"#,
                bin_path = autojump.bin_path(),
                init = autojump.init_for(self),
                ps1 = ps1,
                preinit = preinit,
            ),
        };

        match *self {
            Shell::Fish => {
                fs::create_dir_all(root.join("home/pazi/.config/fish")).unwrap();
                // hack to prevent 'fish_config_interactive' trying to spin off a
                // completions update background process
                fs::create_dir_all(root.join("home/pazi/.local/share/fish/generated_completions"))
                    .unwrap();
            }
            _ => {
                fs::create_dir_all(root.join("home/pazi")).unwrap();
            }
        }
        fs::File::create(rc_file)
            .unwrap()
            .write_all(rc_template.as_bytes())
            .unwrap();
    }

    pub fn command(&self, root: &Path) -> ShellCmd {
        let home = root.join("home/pazi").to_string_lossy().to_string();
        ShellCmd {
            cmd: self.name(),
            env: vec![("HOME", home)],
        }
    }
}
