use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_5639650: FileType = FileType {
    file_format: &FileFormat {
        id: 5_639_650,
        source_type: SourceType::Iana,
        name: "vnd.pco.b16",
        extensions: &[],
        media_types: &["image/vnd.pco.b16"],
        signatures: &[],
        related_formats: &[],
    },
};
