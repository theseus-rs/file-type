use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_79242927: FileType = FileType {
    file_format: &FileFormat {
        id: 79_242_927,
        source_type: SourceType::Wikidata,
        name: "Adobe After Effects Graphics",
        extensions: &["aegraphic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
