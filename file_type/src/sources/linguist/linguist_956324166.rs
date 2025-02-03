use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_956324166: FileFormat = FileFormat {
    id: 956_324_166,
    source_type: SourceType::Linguist,
    name: "Git Attributes",
    extensions: &[],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
