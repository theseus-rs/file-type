use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47167455: FileType = FileType {
    file_format: &FileFormat {
        id: 47_167_455,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Database file format",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
