use super::Autojumper;
use crate::harness::Shell;
use std::env;
use std::path::Path;

pub struct Jump;

impl Autojumper for Jump {
    fn bin_path(&self) -> String {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("build with cargo");
        let bin_path = Path::new(&crate_dir).join(format!("testbins/jump/jump"));

        if !bin_path.exists() {
            panic!("update submodules before running benches");
        }
        bin_path
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    fn init_for(&self, shell: &Shell) -> String {
        match shell {
            &Shell::Bash | &Shell::Zsh => format!(
                r#"
eval "$({} shell {})"
"#,
                self.bin_path(),
                shell.name(),
            ),
            &Shell::Fish => format!(
                r#"
status --is-interactive; and . ({} shell {} | psub)
"#,
                self.bin_path(),
                shell.name(),
            ),
        }
    }

    fn supported_shells(&self) -> Vec<Shell> {
        vec![Shell::Bash, Shell::Zsh, Shell::Fish]
    }

    fn jump_alias(&self) -> &'static str {
        "j"
    }

    fn to_str(&self) -> &'static str {
        "jump"
    }
}
