use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111605948: FileType = FileType {
    file_format: &FileFormat {
        id: 111_605_948,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2020",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
