use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3513566: FileType = FileType {
    file_format: &FileFormat {
        id: 3_513_566,
        source_type: SourceType::Wikidata,
        name: "tab-separated values",
        extensions: &["tab", "tsv"],
        media_types: &["text/tab-separated-values"],
        signatures: &[],
        related_formats: &[],
    },
};
