use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47539022: FileType = FileType {
    file_format: &FileFormat {
        id: 47_539_022,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing Standards File",
        extensions: &["dws"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
