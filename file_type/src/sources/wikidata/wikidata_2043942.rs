use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2043942: FileType = FileType {
    file_format: &FileFormat {
        id: 2_043_942,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format for Engineering",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
