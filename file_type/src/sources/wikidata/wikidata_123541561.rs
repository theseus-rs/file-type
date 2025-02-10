use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123541561: FileType = FileType {
    file_format: &FileFormat {
        id: 123_541_561,
        source_type: SourceType::Wikidata,
        name: "MPEG-4 playlist",
        extensions: &["m4u", "mxu"],
        media_types: &["video/vnd.mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
