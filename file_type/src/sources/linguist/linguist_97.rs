use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_97: FileFormat = FileFormat {
    id: 97,
    source_type: SourceType::Linguist,
    name: "Eagle",
    extensions: &["brd", "sch"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
