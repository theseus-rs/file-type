use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_210: FileFormat = FileFormat {
    id: 210,
    source_type: SourceType::Linguist,
    name: "Logtalk",
    extensions: &["lgt", "logtalk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
