use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3639768131: FileType = FileType {
    file_format: &FileFormat {
        id: 3_639_768_131,
        source_type: SourceType::Iana,
        name: "mbms-msk-response+xml",
        extensions: &[],
        media_types: &["application/mbms-msk-response+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
