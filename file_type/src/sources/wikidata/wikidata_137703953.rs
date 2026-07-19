use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137703953: FileType = FileType {
    file_format: &FileFormat {
        id: 137_703_953,
        source_type: SourceType::Wikidata,
        name: "ANTLR4 grammar file",
        extensions: &["g4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
