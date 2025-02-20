use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3414676129: FileType = FileType {
    file_format: &FileFormat {
        id: 3_414_676_129,
        source_type: SourceType::Iana,
        name: "ocsp-request",
        extensions: &[],
        media_types: &["application/ocsp-request"],
        signatures: &[],
        related_formats: &[],
    },
};
