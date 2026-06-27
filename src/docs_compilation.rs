use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

const STANDARD_DOCS: &[StandardDoc] = &[StandardDoc {
    path: "style.md",
    content: include_str!("../res/style.md"),
}];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DocsSpec {
    standard: Vec<String>,
    local: Vec<PathBuf>,
}

pub struct DocCompiler {
    spec: DocsSpec,
    local_base: PathBuf,
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
    UnknownStandardDoc {
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

struct StandardDoc {
    path: &'static str,
    content: &'static str,
}

impl DocCompiler {
    pub fn from_spec(spec_path: impl AsRef<Path>) -> Result<Self, DocsCompilationError> {
        let spec_path = spec_path.as_ref();
        Ok(Self {
            spec: read_spec(spec_path)?,
            local_base: spec_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf(),
        })
    }

    pub fn run(self, save_to: impl AsRef<Path>) -> Result<(), DocsCompilationError> {
        let save_to = save_to.as_ref();
        let compiled = compile_docs(self.spec, &self.local_base)?;
        fs::write(save_to, compiled).map_err(|source| DocsCompilationError::OutputWrite {
            path: save_to.to_path_buf(),
            source,
        })
    }
}

fn compile_docs(spec: DocsSpec, local_base: &Path) -> Result<String, DocsCompilationError> {
    let mut chunks = Vec::new();

    for path in spec.standard {
        if path == "*" {
            chunks.extend(STANDARD_DOCS.iter().map(|doc| normalize_doc(doc.content)));
        } else {
            let doc = STANDARD_DOCS
                .iter()
                .find(|doc| doc.path == path)
                .ok_or(DocsCompilationError::UnknownStandardDoc { path })?;
            chunks.push(normalize_doc(doc.content));
        }
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
            DocsCompilationError::UnknownStandardDoc { path } => {
                write!(f, "unknown standard doc: {path}")
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
