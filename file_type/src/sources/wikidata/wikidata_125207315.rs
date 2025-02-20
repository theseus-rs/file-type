use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125207315: FileType = FileType {
    file_format: &FileFormat {
        id: 125_207_315,
        source_type: SourceType::Wikidata,
        name: "VYM part",
        extensions: &["vyp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
