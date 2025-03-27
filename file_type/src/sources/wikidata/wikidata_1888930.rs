use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1888930: FileType = FileType {
    file_format: &FileFormat {
        id: 1_888_930,
        source_type: SourceType::Wikidata,
        name: "Vector Markup Language",
        extensions: &["htm", "html"],
        media_types: &["application/vnd.openxmlformats-officedocument.vmlDrawing"],
        signatures: &[],
        related_formats: &[],
    },
};
