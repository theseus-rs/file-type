use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207194: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_194,
        source_type: SourceType::Wikidata,
        name: "QRT Ray Tracer bitmap",
        extensions: &["dis", "qrt", "raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
