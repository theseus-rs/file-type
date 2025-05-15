use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_346966926: FileType = FileType {
    file_format: &FileFormat {
        id: 346_966_926,
        source_type: SourceType::Iana,
        name: "vnd.blockfact.facta",
        extensions: &[],
        media_types: &["audio/vnd.blockfact.facta"],
        signatures: &[],
        related_formats: &[],
    },
};
