use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_824: FileType = FileType {
    file_format: &FileFormat {
        id: 824,
        source_type: SourceType::Pronom,
        name: "EBCDIC-US",
        extensions: &["ebcdic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
