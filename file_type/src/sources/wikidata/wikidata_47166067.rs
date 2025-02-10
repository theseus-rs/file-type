use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47166067: FileType = FileType {
    file_format: &FileFormat {
        id: 47_166_067,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks file format version 2-3",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
