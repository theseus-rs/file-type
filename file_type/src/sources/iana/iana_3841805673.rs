use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3841805673: FileType = FileType {
    file_format: &FileFormat {
        id: 3_841_805_673,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.5gsv2x-local-service-information",
        extensions: &[],
        media_types: &["application/vnd.3gpp.5gsv2x-local-service-information"],
        signatures: &[],
        related_formats: &[],
    },
};
