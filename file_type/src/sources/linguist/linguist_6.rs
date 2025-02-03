use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_6: FileFormat = FileFormat {
    id: 6,
    source_type: SourceType::Linguist,
    name: "APL",
    extensions: &["apl", "dyalog"],
    media_types: &["text/apl"],
    internal_signatures: &[],
    related_formats: &[],
};
