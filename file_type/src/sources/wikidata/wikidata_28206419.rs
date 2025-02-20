use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206419: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_419,
        source_type: SourceType::Wikidata,
        name: "LuraWave JPEG-2000 Code Stream Format",
        extensions: &["jpc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
