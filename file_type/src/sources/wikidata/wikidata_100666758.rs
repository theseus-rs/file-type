use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100666758: FileType = FileType {
    file_format: &FileFormat {
        id: 100_666_758,
        source_type: SourceType::Wikidata,
        name: "Apple iWork Pages, version 9",
        extensions: &["pages"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
