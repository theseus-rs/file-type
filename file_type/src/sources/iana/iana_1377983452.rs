use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1377983452: FileType = FileType {
    file_format: &FileFormat {
        id: 1_377_983_452,
        source_type: SourceType::Iana,
        name: "vnd.apache.arrow.file",
        extensions: &[],
        media_types: &["application/vnd.apache.arrow.file"],
        signatures: &[],
        related_formats: &[],
    },
};
