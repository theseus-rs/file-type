use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2066517752: FileType = FileType {
    file_format: &FileFormat {
        id: 2_066_517_752,
        source_type: SourceType::Iana,
        name: "vnd.oipf.contentaccessstreaming+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.contentaccessstreaming+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
