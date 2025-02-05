use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_159: FileFormat = FileFormat {
    id: 159,
    source_type: SourceType::Linguist,
    name: "Hy",
    extensions: &["hy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
