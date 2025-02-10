use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43974596: FileType = FileType {
    file_format: &FileFormat {
        id: 43_974_596,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (Binary), version R13",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
