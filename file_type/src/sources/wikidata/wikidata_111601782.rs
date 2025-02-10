use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111601782: FileType = FileType {
    file_format: &FileFormat {
        id: 111_601_782,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2018",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
