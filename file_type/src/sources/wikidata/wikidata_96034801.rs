use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96034801: FileType = FileType {
    file_format: &FileFormat {
        id: 96_034_801,
        source_type: SourceType::Wikidata,
        name: "GXL",
        extensions: &["gxl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
