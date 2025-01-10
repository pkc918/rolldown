use std::collections::HashMap;

use napi_derive::napi;
use rustc_hash::FxBuildHasher;

#[napi(object)]
pub struct BindingRemote {
  pub r#type: Option<String>,
  pub name: String,
  pub entry: String,
  pub entry_global_name: Option<String>,
  pub share_scope: Option<String>,
}

impl From<BindingRemote> for rolldown_plugin_module_federation::Remote {
  fn from(value: BindingRemote) -> Self {
    Self {
      r#type: value.r#type,
      name: value.name,
      entry: value.entry,
      entry_global_name: value.entry_global_name,
      share_scope: value.share_scope,
    }
  }
}

#[napi(object)]
pub struct BindingShared {
  pub name: String,
  pub version: Option<String>,
  pub share_scope: Option<String>,
  pub singleton: Option<bool>,
  pub required_version: Option<String>,
  pub strict_version: Option<bool>,
}

impl From<BindingShared> for rolldown_plugin_module_federation::Shared {
  fn from(value: BindingShared) -> Self {
    Self {
      name: value.name,
      version: value.version,
      share_scope: value.share_scope,
      singleton: value.singleton,
      required_version: value.required_version,
      strict_version: value.strict_version,
    }
  }
}

#[napi(object)]
pub struct BindingModuleFederationPluginOption {
  pub name: String,
  pub filename: Option<String>,
  pub expose: HashMap<String, String, FxBuildHasher>,
  pub remotes: HashMap<String, BindingRemote, FxBuildHasher>,
  pub shared: HashMap<String, BindingShared, FxBuildHasher>,
}

impl From<BindingModuleFederationPluginOption>
  for rolldown_plugin_module_federation::ModuleFederationPluginOption
{
  fn from(value: BindingModuleFederationPluginOption) -> Self {
    Self {
      name: value.name,
      filename: value.filename,
      expose: value.expose,
      remotes: value.remotes.into_iter().map(|(k, v)| (k, v.into())).collect(),
      shared: value.shared.into_iter().map(|(k, v)| (k, v.into())).collect(),
    }
  }
}