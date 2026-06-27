use std::{
    fs,
    path::{Path, PathBuf},
};

use fractic_server_error::{define_internal_error, ServerError};
use serde::Deserialize;

define_internal_error!(DocsSpecReadError, "Failed to read docs spec {path}.", { path: &str });
define_internal_error!(DocsSpecParseError, "Failed to parse docs spec {path}.", { path: &str });
define_internal_error!(UnknownStandardDocError, "Unknown standard doc: {path}.", { path: &str });
define_internal_error!(LocalDocReadError, "Failed to read local doc {path}.", { path: &str });
define_internal_error!(DocsOutputWriteError, "Failed to write compiled docs {path}.", { path: &str });

// Standard docs.
// ----------------------------------------------------------------------------

const STANDARD_DOCS: &[StandardDoc] = &[StandardDoc {
    path: "style.md",
    content: include_str!("../res/style.md"),
}];

struct StandardDoc {
    path: &'static str,
    content: &'static str,
}

// Public interface.
// ----------------------------------------------------------------------------

pub struct DocCompiler {
    spec: DocsSpec,
    local_base: PathBuf,
}

impl DocCompiler {
    pub fn from_spec(spec_path: impl AsRef<Path>) -> Result<Self, ServerError> {
        let spec_path = spec_path.as_ref();
        Ok(Self {
            spec: read_spec(spec_path)?,
            local_base: spec_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf(),
        })
    }

    pub fn run(self, save_to: impl AsRef<Path>) -> Result<(), ServerError> {
        let save_to = save_to.as_ref();
        let compiled = compile_docs(self.spec, &self.local_base)?;
        fs::write(save_to, compiled).map_err(|source| {
            let path = save_to.display().to_string();
            DocsOutputWriteError::with_debug(&path, &source)
        })
    }
}

// Compilation.
// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DocsSpec {
    standard: Vec<String>,
    local: Vec<PathBuf>,
}

fn read_spec(spec_path: &Path) -> Result<DocsSpec, ServerError> {
    let path = spec_path.display().to_string();
    let content = fs::read_to_string(spec_path)
        .map_err(|source| DocsSpecReadError::with_debug(&path, &source))?;
    serde_yaml::from_str(&content).map_err(|source| DocsSpecParseError::with_debug(&path, &source))
}

fn compile_docs(spec: DocsSpec, local_base: &Path) -> Result<String, ServerError> {
    let mut chunks = Vec::new();

    for path in spec.standard {
        if path == "*" {
            chunks.extend(STANDARD_DOCS.iter().map(|doc| normalize_doc(doc.content)));
        } else {
            let doc = STANDARD_DOCS
                .iter()
                .find(|doc| doc.path == path)
                .ok_or_else(|| UnknownStandardDocError::new(&path))?;
            chunks.push(normalize_doc(doc.content));
        }
    }

    for path in spec.local {
        let full_path = local_base.join(&path);
        let full_path_string = full_path.display().to_string();
        let content = fs::read_to_string(&full_path)
            .map_err(|source| LocalDocReadError::with_debug(&full_path_string, &source))?;
        chunks.push(normalize_doc(&content));
    }

    Ok(format!("{}\n", chunks.join("\n\n")))
}

fn normalize_doc(content: &str) -> String {
    content.trim().to_string()
}
