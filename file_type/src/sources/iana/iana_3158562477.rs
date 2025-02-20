use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3158562477: FileType = FileType {
    file_format: &FileFormat {
        id: 3_158_562_477,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.current-location-discovery+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.current-location-discovery+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
