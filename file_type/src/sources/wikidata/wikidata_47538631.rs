use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47538631: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_631,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Custom Dictionary",
        extensions: &["cus"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
