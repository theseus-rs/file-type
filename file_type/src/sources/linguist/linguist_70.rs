use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_70: FileFormat = FileFormat {
    id: 70,
    source_type: SourceType::Linguist,
    name: "Cpp-ObjDump",
    extensions: &[
        "c++-objdump",
        "c++objdump",
        "cpp-objdump",
        "cppobjdump",
        "cxx-objdump",
    ],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
