use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111596714: FileType = FileType {
    file_format: &FileFormat {
        id: 111_596_714,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CS 5.5",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
