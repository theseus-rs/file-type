use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123378444: FileType = FileType {
    file_format: &FileFormat {
        id: 123_378_444,
        source_type: SourceType::Wikidata,
        name: "Caligari Amiga file",
        extensions: &["sob"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
