use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_40: FileFormat = FileFormat {
    id: 40,
    source_type: SourceType::Linguist,
    name: "Zeek",
    extensions: &["bro", "zeek"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
