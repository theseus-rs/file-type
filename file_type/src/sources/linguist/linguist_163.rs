use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_163: FileFormat = FileFormat {
    id: 163,
    source_type: SourceType::Linguist,
    name: "INI",
    extensions: &[
        "cfg",
        "cnf",
        "dof",
        "ini",
        "lektorproject",
        "prefs",
        "pro",
        "properties",
        "url",
    ],
    media_types: &["text/x-properties"],
    signatures: &[],
    related_formats: &[],
};
