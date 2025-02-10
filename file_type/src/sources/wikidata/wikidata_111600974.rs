use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111600974: FileType = FileType {
    file_format: &FileFormat {
        id: 111_600_974,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2015",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
