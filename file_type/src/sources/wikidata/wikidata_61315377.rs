use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61315377: FileType = FileType {
    file_format: &FileFormat {
        id: 61_315_377,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version 2.6",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
