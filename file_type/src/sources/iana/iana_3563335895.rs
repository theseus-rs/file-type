use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3563335895: FileType = FileType {
    file_format: &FileFormat {
        id: 3_563_335_895,
        source_type: SourceType::Iana,
        name: "rpki-roa",
        extensions: &[],
        media_types: &["application/rpki-roa"],
        signatures: &[],
        related_formats: &[],
    },
};
