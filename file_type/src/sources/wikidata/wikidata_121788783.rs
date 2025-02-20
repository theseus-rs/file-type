use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121788783: FileType = FileType {
    file_format: &FileFormat {
        id: 121_788_783,
        source_type: SourceType::Wikidata,
        name: "Yamaha PSR Disk Manager File",
        extensions: &["mng"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
