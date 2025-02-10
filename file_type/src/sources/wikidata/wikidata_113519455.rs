use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113519455: FileType = FileType {
    file_format: &FileFormat {
        id: 113_519_455,
        source_type: SourceType::Wikidata,
        name: "PageMaker Mac Document 6.0",
        extensions: &["pm6", "pt6"],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
