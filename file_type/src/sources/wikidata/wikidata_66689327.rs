use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66689327: FileType = FileType {
    file_format: &FileFormat {
        id: 66_689_327,
        source_type: SourceType::Wikidata,
        name: "Access lock files",
        extensions: &["ldb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
