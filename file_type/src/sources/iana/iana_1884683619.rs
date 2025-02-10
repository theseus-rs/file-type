use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1884683619: FileType = FileType {
    file_format: &FileFormat {
        id: 1_884_683_619,
        source_type: SourceType::Iana,
        name: "pgp-signature",
        extensions: &[],
        media_types: &["application/pgp-signature"],
        signatures: &[],
        related_formats: &[],
    },
};
