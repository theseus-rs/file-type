use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117352037: FileType = FileType {
    file_format: &FileFormat {
        id: 117_352_037,
        source_type: SourceType::Wikidata,
        name: "OrCAD project",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
