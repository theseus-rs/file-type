use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50413931: FileType = FileType {
    file_format: &FileFormat {
        id: 50_413_931,
        source_type: SourceType::Wikidata,
        name: "Lightwright 2 Show File",
        extensions: &["lw2"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
