use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_786683730: FileFormat = FileFormat {
    id: 786_683_730,
    source_type: SourceType::Linguist,
    name: "HXML",
    extensions: &["hxml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
