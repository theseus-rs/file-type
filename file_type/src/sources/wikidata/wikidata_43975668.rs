use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43975668: FileType = FileType {
    file_format: &FileFormat {
        id: 43_975_668,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (Binary), version 2000-2002",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
