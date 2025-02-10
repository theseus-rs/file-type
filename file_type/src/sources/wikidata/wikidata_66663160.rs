use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66663160: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_160,
        source_type: SourceType::Wikidata,
        name: "eSuite word processor format",
        extensions: &["html-wp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
