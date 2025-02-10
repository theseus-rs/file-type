use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47539012: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_012,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing Template",
        extensions: &["dwt"],
        media_types: &["application/x-autocad"],
        signatures: &[],
        related_formats: &[],
    },
};
