use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_382: FileFormat = FileFormat {
    id: 382,
    source_type: SourceType::Linguist,
    name: "UnrealScript",
    extensions: &["uc"],
    media_types: &["text/x-java"],
    signatures: &[],
    related_formats: &[],
};
