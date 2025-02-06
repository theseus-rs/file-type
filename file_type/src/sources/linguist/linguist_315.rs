use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_315: FileFormat = FileFormat {
    id: 315,
    source_type: SourceType::Linguist,
    name: "RUNOFF",
    extensions: &["rnh", "rno"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
