use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49619410: FileType = FileType {
    file_format: &FileFormat {
        id: 49_619_410,
        source_type: SourceType::Wikidata,
        name: "Revit Family File",
        extensions: &["rfa"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
