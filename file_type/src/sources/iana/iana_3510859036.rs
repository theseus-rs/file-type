use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3510859036: FileType = FileType {
    file_format: &FileFormat {
        id: 3_510_859_036,
        source_type: SourceType::Iana,
        name: "pkcs7-mime",
        extensions: &[],
        media_types: &["application/pkcs7-mime"],
        signatures: &[],
        related_formats: &[],
    },
};
