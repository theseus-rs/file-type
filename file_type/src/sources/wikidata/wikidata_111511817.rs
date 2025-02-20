use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111511817: FileType = FileType {
    file_format: &FileFormat {
        id: 111_511_817,
        source_type: SourceType::Wikidata,
        name: "Visual Basic Binary Form file",
        extensions: &["frx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
