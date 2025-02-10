use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113481800: FileType = FileType {
    file_format: &FileFormat {
        id: 113_481_800,
        source_type: SourceType::Wikidata,
        name: "602 Text file 1.0-1.51",
        extensions: &["602"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
