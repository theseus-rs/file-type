use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47166177: FileType = FileType {
    file_format: &FileFormat {
        id: 47_166_177,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Drawing file format",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
