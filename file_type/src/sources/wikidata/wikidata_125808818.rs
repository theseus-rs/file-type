use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125808818: FileType = FileType {
    file_format: &FileFormat {
        id: 125_808_818,
        source_type: SourceType::Wikidata,
        name: "B1 Compressed Archive",
        extensions: &["b1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
