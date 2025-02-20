use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111606210: FileType = FileType {
    file_format: &FileFormat {
        id: 111_606_210,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2021",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
