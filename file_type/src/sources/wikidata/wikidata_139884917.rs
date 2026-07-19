use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139884917: FileType = FileType {
    file_format: &FileFormat {
        id: 139_884_917,
        source_type: SourceType::Wikidata,
        name: "PTC Creo View ECAD Package file",
        extensions: &["edz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
