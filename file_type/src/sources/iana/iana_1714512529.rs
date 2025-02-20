use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1714512529: FileType = FileType {
    file_format: &FileFormat {
        id: 1_714_512_529,
        source_type: SourceType::Iana,
        name: "ocsp-response",
        extensions: &[],
        media_types: &["application/ocsp-response"],
        signatures: &[],
        related_formats: &[],
    },
};
