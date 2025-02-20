use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51839112: FileType = FileType {
    file_format: &FileFormat {
        id: 51_839_112,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Film Roll",
        extensions: &["flm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
