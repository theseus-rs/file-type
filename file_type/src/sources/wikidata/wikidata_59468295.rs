use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59468295: FileType = FileType {
    file_format: &FileFormat {
        id: 59_468_295,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Data XPT (Windows)",
        extensions: &["xpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
