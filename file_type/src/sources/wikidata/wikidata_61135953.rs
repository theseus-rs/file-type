use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61135953: FileType = FileType {
    file_format: &FileFormat {
        id: 61_135_953,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2.2",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
