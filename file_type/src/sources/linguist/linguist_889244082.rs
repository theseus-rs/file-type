use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_889244082: FileFormat = FileFormat {
    id: 889_244_082,
    source_type: SourceType::Linguist,
    name: "Odin",
    extensions: &["odin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
