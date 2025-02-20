use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125916061: FileType = FileType {
    file_format: &FileFormat {
        id: 125_916_061,
        source_type: SourceType::Wikidata,
        name: "SolidWorks Material Database File",
        extensions: &["sldmat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
