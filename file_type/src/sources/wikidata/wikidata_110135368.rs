use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110135368: FileType = FileType {
    file_format: &FileFormat {
        id: 110_135_368,
        source_type: SourceType::Wikidata,
        name: "Serif PhotoPlus Image, version 5-X2",
        extensions: &["spp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
