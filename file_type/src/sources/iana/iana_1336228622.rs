use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1336228622: FileType = FileType {
    file_format: &FileFormat {
        id: 1_336_228_622,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-service-config+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-service-config+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
