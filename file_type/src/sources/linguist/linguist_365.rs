use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_365: FileFormat = FileFormat {
    id: 365,
    source_type: SourceType::Linguist,
    name: "TOML",
    extensions: &["toml"],
    media_types: &["text/x-toml"],
    signatures: &[],
    related_formats: &[],
};
