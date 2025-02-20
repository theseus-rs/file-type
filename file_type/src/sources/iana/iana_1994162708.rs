use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1994162708: FileType = FileType {
    file_format: &FileFormat {
        id: 1_994_162_708,
        source_type: SourceType::Iana,
        name: "x-x509-ca-ra-cert",
        extensions: &[],
        media_types: &["application/x-x509-ca-ra-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
