use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_375265331: FileType = FileType {
    file_format: &FileFormat {
        id: 375_265_331,
        source_type: SourceType::Linguist,
        name: "Quake",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
