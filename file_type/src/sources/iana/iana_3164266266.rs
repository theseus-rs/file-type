use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3164266266: FileType = FileType {
    file_format: &FileFormat {
        id: 3_164_266_266,
        source_type: SourceType::Iana,
        name: "cda+xml",
        extensions: &[],
        media_types: &["application/cda+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
