use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117234171: FileType = FileType {
    file_format: &FileFormat {
        id: 117_234_171,
        source_type: SourceType::Wikidata,
        name: "TurboCAD for Windows 3D Model File",
        extensions: &["mdi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
