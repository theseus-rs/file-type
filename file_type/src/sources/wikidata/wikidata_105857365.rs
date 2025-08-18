use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105857365: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_365,
        source_type: SourceType::Wikidata,
        name: "MAME plugin config (JSON)",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
