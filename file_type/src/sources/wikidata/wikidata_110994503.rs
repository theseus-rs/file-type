use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110994503: FileType = FileType {
    file_format: &FileFormat {
        id: 110_994_503,
        source_type: SourceType::Wikidata,
        name: "Micrografx Pictures Publisher",
        extensions: &["pdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
