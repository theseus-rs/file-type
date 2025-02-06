use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_5: FileFormat = FileFormat {
    id: 5,
    source_type: SourceType::Linguist,
    name: "API Blueprint",
    extensions: &["apib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
