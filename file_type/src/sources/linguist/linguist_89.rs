use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_89: FileFormat = FileFormat {
    id: 89,
    source_type: SourceType::Linguist,
    name: "Dockerfile",
    extensions: &["containerfile", "dockerfile"],
    media_types: &["text/x-dockerfile"],
    signatures: &[],
    related_formats: &[],
};
