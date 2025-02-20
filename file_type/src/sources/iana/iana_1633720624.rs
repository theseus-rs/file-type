use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1633720624: FileType = FileType {
    file_format: &FileFormat {
        id: 1_633_720_624,
        source_type: SourceType::Iana,
        name: "tlsrpt+json",
        extensions: &[],
        media_types: &["application/tlsrpt+json"],
        signatures: &[],
        related_formats: &[],
    },
};
