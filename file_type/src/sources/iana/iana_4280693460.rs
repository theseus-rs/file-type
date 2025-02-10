use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4280693460: FileType = FileType {
    file_format: &FileFormat {
        id: 4_280_693_460,
        source_type: SourceType::Iana,
        name: "pem-certificate-chain",
        extensions: &[],
        media_types: &["application/pem-certificate-chain"],
        signatures: &[],
        related_formats: &[],
    },
};
