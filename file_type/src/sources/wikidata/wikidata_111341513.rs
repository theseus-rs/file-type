use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111341513: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_513,
        source_type: SourceType::Wikidata,
        name: "Signed byte (8-bit) data",
        extensions: &["sb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
