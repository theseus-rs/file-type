use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2648: FileType = FileType {
    file_format: &FileFormat {
        id: 2_648,
        source_type: SourceType::Pronom,
        name: "SHA512 File",
        extensions: &["sha512"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
