use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50413749: FileType = FileType {
    file_format: &FileFormat {
        id: 50_413_749,
        source_type: SourceType::Wikidata,
        name: "Lightwright 4 Show File",
        extensions: &["lw4"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
