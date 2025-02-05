use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_186: FileFormat = FileFormat {
    id: 186,
    source_type: SourceType::Linguist,
    name: "KRL",
    extensions: &["krl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
