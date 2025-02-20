use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2429208373: FileType = FileType {
    file_format: &FileFormat {
        id: 2_429_208_373,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.srvcc-ext+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.srvcc-ext+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
