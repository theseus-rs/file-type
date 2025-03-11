use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113469833: FileType = FileType {
    file_format: &FileFormat {
        id: 113_469_833,
        source_type: SourceType::Wikidata,
        name: "PageMaker Mac Document Version 5",
        extensions: &[],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
