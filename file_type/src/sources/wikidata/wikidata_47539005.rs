use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47539005: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_005,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Font Mapping Table",
        extensions: &["fmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
