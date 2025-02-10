use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_64763165: FileType = FileType {
    file_format: &FileFormat {
        id: 64_763_165,
        source_type: SourceType::Wikidata,
        name: "MapPoint Maps file format",
        extensions: &["ptm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
