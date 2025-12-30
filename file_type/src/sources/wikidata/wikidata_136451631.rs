use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136451631: FileType = FileType {
    file_format: &FileFormat {
        id: 136_451_631,
        source_type: SourceType::Wikidata,
        name: "DaVinci Resolve Project File",
        extensions: &["drp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
