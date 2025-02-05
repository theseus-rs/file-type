use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_97358117: FileFormat = FileFormat {
    id: 97_358_117,
    source_type: SourceType::Linguist,
    name: "Futhark",
    extensions: &["fut"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
