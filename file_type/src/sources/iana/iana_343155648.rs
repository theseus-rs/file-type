use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_343155648: FileType = FileType {
    file_format: &FileFormat {
        id: 343_155_648,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.sprov+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.sprov+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
