use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114093986: FileType = FileType {
    file_format: &FileFormat {
        id: 114_093_986,
        source_type: SourceType::Wikidata,
        name: "Sony SML File",
        extensions: &["sml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
