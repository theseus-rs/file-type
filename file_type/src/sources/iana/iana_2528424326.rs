use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2528424326: FileType = FileType {
    file_format: &FileFormat {
        id: 2_528_424_326,
        source_type: SourceType::Iana,
        name: "p2p-overlay+xml",
        extensions: &[],
        media_types: &["application/p2p-overlay+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
