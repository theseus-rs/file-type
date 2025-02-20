use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125692808: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_808,
        source_type: SourceType::Wikidata,
        name: "Pocket Excel Format",
        extensions: &["pxl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
