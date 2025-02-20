use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110135637: FileType = FileType {
    file_format: &FileFormat {
        id: 110_135_637,
        source_type: SourceType::Wikidata,
        name: "Serif PhotoPlus Image, version 5-X3",
        extensions: &["spp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
