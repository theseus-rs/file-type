use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50376371: FileType = FileType {
    file_format: &FileFormat {
        id: 50_376_371,
        source_type: SourceType::Wikidata,
        name: "SHA256 File",
        extensions: &["sha256"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
