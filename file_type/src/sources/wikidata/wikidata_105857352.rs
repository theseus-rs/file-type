use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105857352: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_352,
        source_type: SourceType::Wikidata,
        name: "Vega Visualization",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
