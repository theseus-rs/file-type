use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117223274: FileType = FileType {
    file_format: &FileFormat {
        id: 117_223_274,
        source_type: SourceType::Wikidata,
        name: "LDB File",
        extensions: &["ldb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
