use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2362650979: FileType = FileType {
    file_format: &FileFormat {
        id: 2_362_650_979,
        source_type: SourceType::Iana,
        name: "CN",
        extensions: &[],
        media_types: &["audio/CN"],
        signatures: &[],
        related_formats: &[],
    },
};
