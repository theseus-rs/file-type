use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2701652: FileType = FileType {
    file_format: &FileFormat {
        id: 2_701_652,
        source_type: SourceType::Wikidata,
        name: "BSP",
        extensions: &["bsp"],
        media_types: &["model/vnd.valve.source.compiled-map"],
        signatures: &[],
        related_formats: &[],
    },
};
