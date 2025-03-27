use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55089059: FileType = FileType {
    file_format: &FileFormat {
        id: 55_089_059,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format, version 2013/2014",
        extensions: &["dxf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
