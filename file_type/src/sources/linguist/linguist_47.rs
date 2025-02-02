use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_47: FileFormat = FileFormat {
    id: 47,
    source_type: SourceType::Linguist,
    name: "CMake",
    extensions: &["cmake", "cmake.in"],
    media_types: &["text/x-cmake"],
    internal_signatures: &[],
    related_formats: &[],
};
