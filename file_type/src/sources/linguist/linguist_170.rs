use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_170: FileFormat = FileFormat {
    id: 170,
    source_type: SourceType::Linguist,
    name: "Isabelle",
    extensions: &["thy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
