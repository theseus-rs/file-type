use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61135623: FileType = FileType {
    file_format: &FileFormat {
        id: 61_135_623,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2.1",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
