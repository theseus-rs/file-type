use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4293424542: FileType = FileType {
    file_format: &FileFormat {
        id: 4_293_424_542,
        source_type: SourceType::Iana,
        name: "G726-16",
        extensions: &[],
        media_types: &["audio/G726-16"],
        signatures: &[],
        related_formats: &[],
    },
};
