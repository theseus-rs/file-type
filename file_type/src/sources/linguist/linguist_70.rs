use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_70: FileType = FileType {
    file_format: &FileFormat {
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
    },
};
