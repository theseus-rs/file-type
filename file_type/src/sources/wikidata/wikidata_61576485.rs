use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61576485: FileType = FileType {
    file_format: &FileFormat {
        id: 61_576_485,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange File Format (ASCII), version R13",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[],
        related_formats: &[],
    },
};
