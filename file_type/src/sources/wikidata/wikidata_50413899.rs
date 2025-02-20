use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50413899: FileType = FileType {
    file_format: &FileFormat {
        id: 50_413_899,
        source_type: SourceType::Wikidata,
        name: "Lightwright 3 Show File",
        extensions: &["lw3"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
