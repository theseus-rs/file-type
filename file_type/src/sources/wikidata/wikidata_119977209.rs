use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119977209: FileType = FileType {
    file_format: &FileFormat {
        id: 119_977_209,
        source_type: SourceType::Wikidata,
        name: "MotionArtist Document",
        extensions: &["fmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
