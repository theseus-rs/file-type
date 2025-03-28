use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1813900545: FileType = FileType {
    file_format: &FileFormat {
        id: 1_813_900_545,
        source_type: SourceType::Iana,
        name: "x-x509-next-ca-cert",
        extensions: &[],
        media_types: &["application/x-x509-next-ca-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
