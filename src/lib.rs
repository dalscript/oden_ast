// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

#[cfg(feature = "cjs")]
mod cjs_parse;
mod comments;
mod lexing;
mod media_type;
mod parsed_source;
mod parsing;
#[cfg(feature = "scopes")]
mod scopes;
mod text_changes;
mod text_encoding;
mod text_info;
#[cfg(feature = "transpiling")]
mod transpiling;
mod types;

#[cfg(feature = "view")]
pub use dprint_swc_ecma_ast_view as view;

#[cfg(feature = "cjs")]
pub use cjs_parse::*;
pub use comments::*;
pub use lexing::*;
pub use media_type::*;
pub use parsed_source::*;
pub use parsing::*;
#[cfg(feature = "scopes")]
pub use scopes::*;
pub use text_changes::*;
pub use text_info::*;
#[cfg(feature = "transpiling")]
pub use transpiling::*;
pub use types::*;

pub mod swc {
  pub use swc_atoms as atoms;
  #[cfg(feature = "bundler")]
  pub use swc_bundler as bundler;
  pub use swc_common as common;
  pub use swc_ecmascript::ast;
  #[cfg(feature = "codegen")]
  pub use swc_ecmascript::codegen;
  #[cfg(feature = "dep_graph")]
  pub use swc_ecmascript::dep_graph;
  #[cfg(feature = "minifier")]
  pub use swc_ecmascript::minifier;
  pub use swc_ecmascript::parser;
  #[cfg(feature = "preset_env")]
  pub use swc_ecmascript::preset_env;
  #[cfg(feature = "transforms")]
  pub use swc_ecmascript::transforms;
  #[cfg(feature = "utils")]
  pub use swc_ecmascript::utils;
  #[cfg(feature = "visit")]
  pub use swc_ecmascript::visit;
}
