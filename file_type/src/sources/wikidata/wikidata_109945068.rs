use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109945068: FileType = FileType {
    file_format: &FileFormat {
        id: 109_945_068,
        source_type: SourceType::Wikidata,
        name: "Goo Document file format",
        extensions: &["goo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
