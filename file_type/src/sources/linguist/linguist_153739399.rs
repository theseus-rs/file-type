use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_153739399: FileType = FileType {
    file_format: &FileFormat {
        id: 153_739_399,
        source_type: SourceType::Linguist,
        name: "OpenQASM",
        extensions: &["qasm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
