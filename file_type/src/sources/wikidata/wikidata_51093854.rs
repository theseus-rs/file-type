use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51093854: FileType = FileType {
    file_format: &FileFormat {
        id: 51_093_854,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Plot Configuration File, version 2000",
        extensions: &["pc3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
