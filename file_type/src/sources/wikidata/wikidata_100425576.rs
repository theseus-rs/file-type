use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100425576: FileType = FileType {
    file_format: &FileFormat {
        id: 100_425_576,
        source_type: SourceType::Wikidata,
        name: "Minitab Worksheet, version 12",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
