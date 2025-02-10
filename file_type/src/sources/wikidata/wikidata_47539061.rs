use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47539061: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_061,
        source_type: SourceType::Wikidata,
        name: "AutoCAD dbConnect Template Set",
        extensions: &["dbt"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
