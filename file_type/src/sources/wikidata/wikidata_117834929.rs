use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117834929: FileType = FileType {
    file_format: &FileFormat {
        id: 117_834_929,
        source_type: SourceType::Wikidata,
        name: "AT&T Group 4 file",
        extensions: &["att"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
