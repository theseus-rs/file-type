use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123369914: FileType = FileType {
    file_format: &FileFormat {
        id: 123_369_914,
        source_type: SourceType::Wikidata,
        name: "High-Throughput JPEG 2000",
        extensions: &["jph"],
        media_types: &["image/jph"],
        signatures: &[],
        related_formats: &[],
    },
};
