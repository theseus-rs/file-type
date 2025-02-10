use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49619991: FileType = FileType {
    file_format: &FileFormat {
        id: 49_619_991,
        source_type: SourceType::Wikidata,
        name: "Revit External Group",
        extensions: &["rvg"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
