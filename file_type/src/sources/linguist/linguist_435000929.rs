use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_435000929: FileFormat = FileFormat {
    id: 435_000_929,
    source_type: SourceType::Linguist,
    name: "DenizenScript",
    extensions: &["dsc"],
    media_types: &["text/x-yaml"],
    signatures: &[],
    related_formats: &[],
};
