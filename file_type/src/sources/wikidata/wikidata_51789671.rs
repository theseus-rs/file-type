use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51789671: FileType = FileType {
    file_format: &FileFormat {
        id: 51_789_671,
        source_type: SourceType::Wikidata,
        name: "AutoCAD External Database Configuration File",
        extensions: &["udl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
