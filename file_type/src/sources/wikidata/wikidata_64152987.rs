use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_64152987: FileType = FileType {
    file_format: &FileFormat {
        id: 64_152_987,
        source_type: SourceType::Wikidata,
        name: "Crossword file format",
        extensions: &["xd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
