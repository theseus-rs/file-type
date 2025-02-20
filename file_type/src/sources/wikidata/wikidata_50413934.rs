use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50413934: FileType = FileType {
    file_format: &FileFormat {
        id: 50_413_934,
        source_type: SourceType::Wikidata,
        name: "Lightwright 1 Show File",
        extensions: &["lw1"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
