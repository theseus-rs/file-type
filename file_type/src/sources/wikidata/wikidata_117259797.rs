use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117259797: FileType = FileType {
    file_format: &FileFormat {
        id: 117_259_797,
        source_type: SourceType::Wikidata,
        name: "TurboCAD 3D Model file",
        extensions: &["mdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
