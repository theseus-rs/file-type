use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_441833144: FileType = FileType {
    file_format: &FileFormat {
        id: 441_833_144,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.5gsa2x-local-service-information",
        extensions: &[],
        media_types: &["application/vnd.3gpp.5gsa2x-local-service-information"],
        signatures: &[],
        related_formats: &[],
    },
};
