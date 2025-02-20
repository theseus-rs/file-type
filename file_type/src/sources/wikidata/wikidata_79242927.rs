use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
