use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100165244: FileType = FileType {
    file_format: &FileFormat {
        id: 100_165_244,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format (ASCII), version 2018/2019/2020",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
