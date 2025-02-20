use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117855462: FileType = FileType {
    file_format: &FileFormat {
        id: 117_855_462,
        source_type: SourceType::Wikidata,
        name: "PRODUCT R&D Fax File",
        extensions: &["prd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
