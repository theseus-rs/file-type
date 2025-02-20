use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_208700028: FileType = FileType {
    file_format: &FileFormat {
        id: 208_700_028,
        source_type: SourceType::Linguist,
        name: "X Font Directory Index",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
