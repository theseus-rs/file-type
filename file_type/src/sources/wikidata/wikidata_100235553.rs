use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100235553: FileType = FileType {
    file_format: &FileFormat {
        id: 100_235_553,
        source_type: SourceType::Wikidata,
        name: "FARO Laser Scan File",
        extensions: &["fls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
