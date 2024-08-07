# Scaleway Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/scaleway)](https://pkg.fluentci.io/scaleway)
[![ci](https://github.com/fluentci-io/scaleway-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/scaleway-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [Scaleway CLI](https://www.scaleway.com/en/cli/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm scaleway setup
```

## Functions

| Name   | Description                                 |
| ------ | ------------------------------------------- |
| setup  | Installs a specific version of scaleway cli.|

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/scaleway@v0.1.0?wasm=1", "setup", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: scaleway
    args: |
      setup
  env:
    GITHUB_ACCESS_TOKEN: ${{ secrets.GITHUB_TOKEN }}
- name: Show scaleway CLI version
  run: scw version
```
