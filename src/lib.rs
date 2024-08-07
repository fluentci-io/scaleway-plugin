use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(_: String) -> FnResult<String> {
    let os = dag().get_os()?;
    let arch = dag().get_arch()?;

    match os.as_str() {
        "macos" => {
            dag().set_envs(vec![("OS".into(), "darwin".into())])?;
        }
        _ => dag().set_envs(vec![("OS".into(), os)])?,
    }

    match arch.as_str() {
        "x86_64" => {
            dag().set_envs(vec![("ARCH".into(), "amd64".into())])?;
        }
        "aarch64" => {
            dag().set_envs(vec![("ARCH".into(), "arm64".into())])?;
        }
        _ => dag().set_envs(vec![("ARCH".into(), arch)])?,
    }
    let version = dag().get_env("SCW_VERSION")?;

    if version.is_empty() {
        dag().set_envs(vec![("SCW_VERSION".into(), "2.32.1".into())])?;
    }

    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["type scw > /dev/null 2> /dev/null || pkgx wget https://github.com/scaleway/scaleway-cli/releases/download/v${SCW_VERSION}/scaleway-cli_${SCW_VERSION}_${OS}_${ARCH}"])?
        .with_exec(vec!["type scw > /dev/null 2> /dev/null || chmod a+x scaleway-cli_${SCW_VERSION}_${OS}_${ARCH}"])?
        .with_exec(vec!["mkdir -p $HOME/.local/bin"])?
        .with_exec(vec!["type scw > /dev/null 2> /dev/null || mv scaleway-cli_${SCW_VERSION}_${OS}_${ARCH} $HOME/.local/bin/scw"])?
        .stdout()?;

    Ok(stdout)
}
