use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4273659658: FileType = FileType {
    file_format: &FileFormat {
        id: 4_273_659_658,
        source_type: SourceType::Iana,
        name: "ST2110-41",
        extensions: &[],
        media_types: &["application/ST2110-41"],
        signatures: &[],
        related_formats: &[],
    },
};
