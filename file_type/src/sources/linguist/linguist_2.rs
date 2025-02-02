use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_2: FileFormat = FileFormat {
    id: 2,
    source_type: SourceType::Linguist,
    name: "AGS Script",
    extensions: &["asc", "ash"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
