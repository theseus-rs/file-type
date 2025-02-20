use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59851255: FileType = FileType {
    file_format: &FileFormat {
        id: 59_851_255,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII)",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
