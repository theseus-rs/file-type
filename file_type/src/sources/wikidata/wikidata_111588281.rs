use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111588281: FileType = FileType {
    file_format: &FileFormat {
        id: 111_588_281,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Library 2",
        extensions: &["indl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
