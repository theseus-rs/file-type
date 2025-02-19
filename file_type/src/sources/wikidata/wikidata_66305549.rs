use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66305549: FileType = FileType {
    file_format: &FileFormat {
        id: 66_305_549,
        source_type: SourceType::Wikidata,
        name: "Splitted AVI File",
        extensions: &["nvavi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
