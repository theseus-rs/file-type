use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139884782: FileType = FileType {
    file_format: &FileFormat {
        id: 139_884_782,
        source_type: SourceType::Wikidata,
        name: "PTC Creo Viewable Session",
        extensions: &["pvs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
