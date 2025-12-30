use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60558690: FileType = FileType {
    file_format: &FileFormat {
        id: 60_558_690,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Database, version 2-3",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
