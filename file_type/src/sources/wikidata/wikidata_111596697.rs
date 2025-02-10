use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111596697: FileType = FileType {
    file_format: &FileFormat {
        id: 111_596_697,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version 2",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
