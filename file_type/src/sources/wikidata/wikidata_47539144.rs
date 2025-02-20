use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47539144: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_144,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Batch Plot File, version 2000-2005",
        extensions: &["bp3"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
