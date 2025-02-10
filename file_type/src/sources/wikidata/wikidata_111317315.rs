use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111317315: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_315,
        source_type: SourceType::Wikidata,
        name: "iPhone ring-tone",
        extensions: &["m4r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
