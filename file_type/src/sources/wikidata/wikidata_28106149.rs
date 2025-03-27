use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28106149: FileType = FileType {
    file_format: &FileFormat {
        id: 28_106_149,
        source_type: SourceType::Wikidata,
        name: "PIC",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
