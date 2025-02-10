use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47166297: FileType = FileType {
    file_format: &FileFormat {
        id: 47_166_297,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Word Processor file format",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
