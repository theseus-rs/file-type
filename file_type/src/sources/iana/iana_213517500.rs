use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_213517500: FileType = FileType {
    file_format: &FileFormat {
        id: 213_517_500,
        source_type: SourceType::Iana,
        name: "vnd.mophun.certificate",
        extensions: &[],
        media_types: &["application/vnd.mophun.certificate"],
        signatures: &[],
        related_formats: &[],
    },
};
