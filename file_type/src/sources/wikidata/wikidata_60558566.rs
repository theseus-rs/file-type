use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60558566: FileType = FileType {
    file_format: &FileFormat {
        id: 60_558_566,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Word Processor",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
