use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960099: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_099,
        source_type: SourceType::Wikidata,
        name: "Stems",
        extensions: &["stem.mp4"],
        media_types: &["video/audio"],
        signatures: &[],
        related_formats: &[],
    },
};
