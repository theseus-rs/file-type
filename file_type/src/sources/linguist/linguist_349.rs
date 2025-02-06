use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_349: FileFormat = FileFormat {
    id: 349,
    source_type: SourceType::Linguist,
    name: "Slash",
    extensions: &["sl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
