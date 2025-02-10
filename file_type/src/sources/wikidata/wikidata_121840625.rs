use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121840625: FileType = FileType {
    file_format: &FileFormat {
        id: 121_840_625,
        source_type: SourceType::Wikidata,
        name: "Common Instrument File 1",
        extensions: &["ci1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
