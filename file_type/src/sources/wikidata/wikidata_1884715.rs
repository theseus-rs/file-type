use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1884715: FileType = FileType {
    file_format: &FileFormat {
        id: 1_884_715,
        source_type: SourceType::Wikidata,
        name: "Video Object",
        extensions: &["vob"],
        media_types: &["video/dvd", "video/mpeg", "video/x-ms-vob"],
        signatures: &[],
        related_formats: &[],
    },
};
