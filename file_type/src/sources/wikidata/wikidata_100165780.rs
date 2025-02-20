use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100165780: FileType = FileType {
    file_format: &FileFormat {
        id: 100_165_780,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format (Binary), version 2018-2021",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
