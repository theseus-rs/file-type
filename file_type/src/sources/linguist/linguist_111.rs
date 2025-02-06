use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_111: FileFormat = FileFormat {
    id: 111,
    source_type: SourceType::Linguist,
    name: "Filebench WML",
    extensions: &["f"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
