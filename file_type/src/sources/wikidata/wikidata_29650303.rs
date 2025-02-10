use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650303: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_303,
        source_type: SourceType::Wikidata,
        name: "PSRFITS",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
