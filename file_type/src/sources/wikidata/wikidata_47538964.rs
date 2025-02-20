use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
