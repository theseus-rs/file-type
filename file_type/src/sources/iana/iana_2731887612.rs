use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2731887612: FileType = FileType {
    file_format: &FileFormat {
        id: 2_731_887_612,
        source_type: SourceType::Iana,
        name: "pkcs8-encrypted",
        extensions: &[],
        media_types: &["application/pkcs8-encrypted"],
        signatures: &[],
        related_formats: &[],
    },
};
