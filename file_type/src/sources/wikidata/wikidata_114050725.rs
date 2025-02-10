use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114050725: FileType = FileType {
    file_format: &FileFormat {
        id: 114_050_725,
        source_type: SourceType::Wikidata,
        name: "Canon CIF File",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
