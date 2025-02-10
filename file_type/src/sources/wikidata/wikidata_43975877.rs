use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43975877: FileType = FileType {
    file_format: &FileFormat {
        id: 43_975_877,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (Binary), version 2004-2005",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
