use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117707128: FileType = FileType {
    file_format: &FileFormat {
        id: 117_707_128,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Project file",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
