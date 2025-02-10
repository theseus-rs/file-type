use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_636189500: FileType = FileType {
    file_format: &FileFormat {
        id: 636_189_500,
        source_type: SourceType::Iana,
        name: "vnd.apothekende.reservation+json",
        extensions: &[],
        media_types: &["application/vnd.apothekende.reservation+json"],
        signatures: &[],
        related_formats: &[],
    },
};
