use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100165626: FileType = FileType {
    file_format: &FileFormat {
        id: 100_165_626,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format (Binary), version 2013-2017",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
