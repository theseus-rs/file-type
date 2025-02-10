use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61707627: FileType = FileType {
    file_format: &FileFormat {
        id: 61_707_627,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Database File Locking Information",
        extensions: &["dwl"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
