use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(_: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx curl -s https://raw.githubusercontent.com/scaleway/scaleway-cli/master/scripts/get.sh | sh"])?
        .stdout()?;

    Ok(stdout)
}
