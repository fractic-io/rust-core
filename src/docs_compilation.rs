use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

const CORE_DOCS: &[CoreDoc] = &[CoreDoc {
    path: "rust-style.md",
    content: include_str!("../res/rust-style.md"),
}];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DocsSpec {
    pub fractic_core: Vec<String>,
    pub local: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum DocsCompilationError {
    SpecRead {
        path: PathBuf,
        source: io::Error,
    },
    SpecParse {
        path: PathBuf,
        source: serde_yaml::Error,
    },
    UnknownCoreDoc {
        path: String,
    },
    LocalDocRead {
        path: PathBuf,
        source: io::Error,
    },
    OutputWrite {
        path: PathBuf,
        source: io::Error,
    },
}

struct CoreDoc {
    path: &'static str,
    content: &'static str,
}

pub fn compile_docs_from_spec_file(
    spec_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), DocsCompilationError> {
    let spec_path = spec_path.as_ref();
    let output_path = output_path.as_ref();
    let spec = read_spec(spec_path)?;
    let compiled = compile_docs(spec, spec_path.parent().unwrap_or_else(|| Path::new(".")))?;
    fs::write(output_path, compiled).map_err(|source| DocsCompilationError::OutputWrite {
        path: output_path.to_path_buf(),
        source,
    })
}

pub fn compile_docs(spec: DocsSpec, local_base: &Path) -> Result<String, DocsCompilationError> {
    let mut chunks = Vec::new();

    for path in spec.fractic_core {
        let doc = CORE_DOCS
            .iter()
            .find(|doc| doc.path == path)
            .ok_or(DocsCompilationError::UnknownCoreDoc { path })?;
        chunks.push(normalize_doc(doc.content));
    }

    for path in spec.local {
        let full_path = local_base.join(&path);
        let content = fs::read_to_string(&full_path).map_err(|source| {
            DocsCompilationError::LocalDocRead {
                path: full_path,
                source,
            }
        })?;
        chunks.push(normalize_doc(&content));
    }

    Ok(format!("{}\n", chunks.join("\n\n")))
}

fn read_spec(spec_path: &Path) -> Result<DocsSpec, DocsCompilationError> {
    let content =
        fs::read_to_string(spec_path).map_err(|source| DocsCompilationError::SpecRead {
            path: spec_path.to_path_buf(),
            source,
        })?;
    serde_yaml::from_str(&content).map_err(|source| DocsCompilationError::SpecParse {
        path: spec_path.to_path_buf(),
        source,
    })
}

fn normalize_doc(content: &str) -> String {
    content.trim().to_string()
}

impl fmt::Display for DocsCompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocsCompilationError::SpecRead { path, source } => {
                write!(f, "failed to read docs spec {}: {source}", path.display())
            }
            DocsCompilationError::SpecParse { path, source } => {
                write!(f, "failed to parse docs spec {}: {source}", path.display())
            }
            DocsCompilationError::UnknownCoreDoc { path } => {
                write!(f, "unknown fractic-core doc: {path}")
            }
            DocsCompilationError::LocalDocRead { path, source } => {
                write!(f, "failed to read local doc {}: {source}", path.display())
            }
            DocsCompilationError::OutputWrite { path, source } => {
                write!(
                    f,
                    "failed to write compiled docs {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for DocsCompilationError {}
