use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_132155960: FileType = FileType {
    file_format: &FileFormat {
        id: 132_155_960,
        source_type: SourceType::Wikidata,
        name: "Braille text file format",
        extensions: &["brl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
