use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67206681: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_681,
        source_type: SourceType::Wikidata,
        name: "TurboCAD Template",
        extensions: &["tct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
