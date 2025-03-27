use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7455595: FileType = FileType {
    file_format: &FileFormat {
        id: 7_455_595,
        source_type: SourceType::Wikidata,
        name: "Server Normal Format",
        extensions: &["snf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
