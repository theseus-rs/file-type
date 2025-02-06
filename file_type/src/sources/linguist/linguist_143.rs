use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_143: FileFormat = FileFormat {
    id: 143,
    source_type: SourceType::Linguist,
    name: "Groovy Server Pages",
    extensions: &["gsp"],
    media_types: &["application/x-jsp"],
    signatures: &[],
    related_formats: &[],
};
