use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3431652971: FileType = FileType {
    file_format: &FileFormat {
        id: 3_431_652_971,
        source_type: SourceType::Iana,
        name: "vnd.nuera.ecelp7470",
        extensions: &[],
        media_types: &["audio/vnd.nuera.ecelp7470"],
        signatures: &[],
        related_formats: &[],
    },
};
