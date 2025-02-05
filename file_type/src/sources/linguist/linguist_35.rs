use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_35: FileFormat = FileFormat {
    id: 35,
    source_type: SourceType::Linguist,
    name: "BlitzMax",
    extensions: &["bmx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
