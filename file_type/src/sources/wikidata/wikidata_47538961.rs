use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47538961: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_961,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Menu Resource File",
        extensions: &["mnr", "mnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
