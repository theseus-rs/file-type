use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_306: FileFormat = FileFormat {
    id: 306,
    source_type: SourceType::Linguist,
    name: "QMake",
    extensions: &["pri", "pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
