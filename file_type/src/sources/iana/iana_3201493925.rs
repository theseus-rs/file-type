use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3201493925: FileType = FileType {
    file_format: &FileFormat {
        id: 3_201_493_925,
        source_type: SourceType::Iana,
        name: "pgp-encrypted",
        extensions: &[],
        media_types: &["application/pgp-encrypted"],
        signatures: &[],
        related_formats: &[],
    },
};
