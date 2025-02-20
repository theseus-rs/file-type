use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_249: FileType = FileType {
    file_format: &FileFormat {
        id: 249,
        source_type: SourceType::Linguist,
        name: "Nim",
        extensions: &["nim", "nim.cfg", "nimble", "nimrod", "nims"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
