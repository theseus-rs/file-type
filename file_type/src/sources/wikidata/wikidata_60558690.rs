use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60558690: FileType = FileType {
    file_format: &FileFormat {
        id: 60_558_690,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Database, version 2",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
