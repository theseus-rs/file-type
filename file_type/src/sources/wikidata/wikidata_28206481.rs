use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206481: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_481,
        source_type: SourceType::Wikidata,
        name: "Krita document",
        extensions: &["kra"],
        media_types: &["application/x-krita"],
        signatures: &[],
        related_formats: &[],
    },
};
