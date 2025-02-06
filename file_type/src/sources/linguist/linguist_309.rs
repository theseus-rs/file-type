use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_309: FileFormat = FileFormat {
    id: 309,
    source_type: SourceType::Linguist,
    name: "RDoc",
    extensions: &["rdoc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
