use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61053201: FileType = FileType {
    file_format: &FileFormat {
        id: 61_053_201,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 1.3",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
