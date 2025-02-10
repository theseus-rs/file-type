use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4161761819: FileType = FileType {
    file_format: &FileFormat {
        id: 4_161_761_819,
        source_type: SourceType::Iana,
        name: "fdf",
        extensions: &[],
        media_types: &["application/fdf"],
        signatures: &[],
        related_formats: &[],
    },
};
