use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4083711601: FileType = FileType {
    file_format: &FileFormat {
        id: 4_083_711_601,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-user-profile+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-user-profile+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
