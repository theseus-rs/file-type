use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_42855167: FileType = FileType {
    file_format: &FileFormat {
        id: 42_855_167,
        source_type: SourceType::Wikidata,
        name: "Google Video File",
        extensions: &["gvi"],
        media_types: &["video/gvi"],
        signatures: &[],
        related_formats: &[],
    },
};
