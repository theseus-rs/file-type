use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_452082346: FileType = FileType {
    file_format: &FileFormat {
        id: 452_082_346,
        source_type: SourceType::Iana,
        name: "pgp-keys",
        extensions: &[],
        media_types: &["application/pgp-keys"],
        signatures: &[],
        related_formats: &[],
    },
};
