use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
