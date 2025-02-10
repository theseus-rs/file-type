use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47538964: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_964,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Source Menu File",
        extensions: &["mns"],
        media_types: &["application/x-autocad"],
        signatures: &[],
        related_formats: &[],
    },
};
