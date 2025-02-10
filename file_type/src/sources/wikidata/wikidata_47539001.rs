use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47539001: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_001,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Linetype Definition File",
        extensions: &["lin"],
        media_types: &["application/x-autocad"],
        signatures: &[],
        related_formats: &[],
    },
};
