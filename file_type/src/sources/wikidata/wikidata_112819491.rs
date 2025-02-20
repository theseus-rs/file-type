use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112819491: FileType = FileType {
    file_format: &FileFormat {
        id: 112_819_491,
        source_type: SourceType::Wikidata,
        name: "Acclaim mocap file",
        extensions: &["amc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
