use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551401: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_401,
        source_type: SourceType::Wikidata,
        name: "Adobe Separation Table File",
        extensions: &["ast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
