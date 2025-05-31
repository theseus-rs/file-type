use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1937543065: FileType = FileType {
    file_format: &FileFormat {
        id: 1_937_543_065,
        source_type: SourceType::Iana,
        name: "rs-metadata+xml",
        extensions: &[],
        media_types: &["application/rs-metadata+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
