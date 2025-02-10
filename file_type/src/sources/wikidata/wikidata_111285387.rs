use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111285387: FileType = FileType {
    file_format: &FileFormat {
        id: 111_285_387,
        source_type: SourceType::Wikidata,
        name: "Sound Tools HCOM format",
        extensions: &["hcom"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
