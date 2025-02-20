use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47516360: FileType = FileType {
    file_format: &FileFormat {
        id: 47_516_360,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Catalog XPT (Unix) v.9.1",
        extensions: &["xpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
