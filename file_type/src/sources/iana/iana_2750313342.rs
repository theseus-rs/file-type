use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2750313342: FileType = FileType {
    file_format: &FileFormat {
        id: 2_750_313_342,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.imd+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.imd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
