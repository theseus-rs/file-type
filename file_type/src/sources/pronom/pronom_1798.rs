use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1798: FileType = FileType {
    file_format: &FileFormat {
        id: 1_798,
        source_type: SourceType::Pronom,
        name: "MD5 File",
        extensions: &["md5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
