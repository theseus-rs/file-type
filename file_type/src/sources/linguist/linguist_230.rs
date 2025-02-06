use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_230: FileFormat = FileFormat {
    id: 230,
    source_type: SourceType::Linguist,
    name: "Metal",
    extensions: &["metal"],
    media_types: &["text/x-c++src"],
    signatures: &[],
    related_formats: &[],
};
