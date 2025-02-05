use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_386: FileFormat = FileFormat {
    id: 386,
    source_type: SourceType::Linguist,
    name: "Vala",
    extensions: &["vala", "vapi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
