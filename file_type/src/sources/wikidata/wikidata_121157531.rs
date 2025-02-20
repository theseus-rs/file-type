use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121157531: FileType = FileType {
    file_format: &FileFormat {
        id: 121_157_531,
        source_type: SourceType::Wikidata,
        name: "FloorPlan 3D Template",
        extensions: &["fpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
