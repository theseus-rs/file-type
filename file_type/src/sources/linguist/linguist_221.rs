use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_221: FileFormat = FileFormat {
    id: 221,
    source_type: SourceType::Linguist,
    name: "Mako",
    extensions: &["mako", "mao"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
