use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_351: FileFormat = FileFormat {
    id: 351,
    source_type: SourceType::Linguist,
    name: "Smali",
    extensions: &["smali"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
