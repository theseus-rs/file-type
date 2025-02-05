use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_77: FileFormat = FileFormat {
    id: 77,
    source_type: SourceType::Linguist,
    name: "Cuda",
    extensions: &["cu", "cuh"],
    media_types: &["text/x-c++src"],
    signatures: &[],
    related_formats: &[],
};
