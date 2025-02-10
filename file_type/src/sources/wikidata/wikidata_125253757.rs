use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125253757: FileType = FileType {
    file_format: &FileFormat {
        id: 125_253_757,
        source_type: SourceType::Wikidata,
        name: "Cytoscape Exchange Format",
        extensions: &["cx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
