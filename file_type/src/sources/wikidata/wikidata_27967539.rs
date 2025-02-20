use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967539: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_539,
        source_type: SourceType::Wikidata,
        name: "Advanced Video Coding",
        extensions: &["mp4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
