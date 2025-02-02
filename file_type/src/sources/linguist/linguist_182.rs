use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_182: FileFormat = FileFormat {
    id: 182,
    source_type: SourceType::Linguist,
    name: "Java Server Pages",
    extensions: &["jsp", "tag"],
    media_types: &["application/x-jsp"],
    internal_signatures: &[],
    related_formats: &[],
};
