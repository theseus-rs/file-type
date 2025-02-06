use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_407: FileFormat = FileFormat {
    id: 407,
    source_type: SourceType::Linguist,
    name: "YAML",
    extensions: &[
        "mir",
        "reek",
        "rviz",
        "sublime-syntax",
        "syntax",
        "yaml",
        "yaml-tmlanguage",
        "yaml.sed",
        "yml",
        "yml.mysql",
    ],
    media_types: &["text/x-yaml"],
    signatures: &[],
    related_formats: &[],
};
