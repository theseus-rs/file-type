use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50414080: FileType = FileType {
    file_format: &FileFormat {
        id: 50_414_080,
        source_type: SourceType::Wikidata,
        name: "Lightwright 6 Show File",
        extensions: &["lw6"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
