use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_411: FileFormat = FileFormat {
    id: 411,
    source_type: SourceType::Linguist,
    name: "Zimpl",
    extensions: &["zimpl", "zmpl", "zpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
