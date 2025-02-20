use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1797: FileType = FileType {
    file_format: &FileFormat {
        id: 1_797,
        source_type: SourceType::Pronom,
        name: "SHA1 File",
        extensions: &["sha1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
