use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_321684729: FileFormat = FileFormat {
    id: 321_684_729,
    source_type: SourceType::Linguist,
    name: "CODEOWNERS",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
