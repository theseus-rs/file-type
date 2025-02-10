use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100165439: FileType = FileType {
    file_format: &FileFormat {
        id: 100_165_439,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format (Binary), version 2007-2009",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
