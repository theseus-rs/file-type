use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100166033: FileType = FileType {
    file_format: &FileFormat {
        id: 100_166_033,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Format (Binary) (Generic)",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
