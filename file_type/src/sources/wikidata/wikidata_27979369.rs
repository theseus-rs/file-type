use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979369: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_369,
        source_type: SourceType::Wikidata,
        name: "ReSample",
        extensions: &["srs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
