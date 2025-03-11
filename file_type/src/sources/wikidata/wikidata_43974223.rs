use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_43974223: FileType = FileType {
    file_format: &FileFormat {
        id: 43_974_223,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (Binary), version R11/12",
        extensions: &[],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
