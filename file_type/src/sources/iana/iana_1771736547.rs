use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1771736547: FileType = FileType {
    file_format: &FileFormat {
        id: 1_771_736_547,
        source_type: SourceType::Iana,
        name: "vnd.bbf.usp.error",
        extensions: &[],
        media_types: &["application/vnd.bbf.usp.error"],
        signatures: &[],
        related_formats: &[],
    },
};
