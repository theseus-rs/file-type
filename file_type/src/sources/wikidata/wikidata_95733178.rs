use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_95733178: FileType = FileType {
    file_format: &FileFormat {
        id: 95_733_178,
        source_type: SourceType::Wikidata,
        name: "RealAudio version 4",
        extensions: &["ra"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
