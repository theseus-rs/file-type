use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205445: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_445,
        source_type: SourceType::Wikidata,
        name: "X3F",
        extensions: &["x3f"],
        media_types: &["image/x-raw-sigma"],
        signatures: &[],
        related_formats: &[],
    },
};
