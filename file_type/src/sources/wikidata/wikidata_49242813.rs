use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49242813: FileType = FileType {
    file_format: &FileFormat {
        id: 49_242_813,
        source_type: SourceType::Wikidata,
        name: "HTML Extension File",
        extensions: &["htx"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
