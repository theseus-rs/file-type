use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_120155765: FileType = FileType {
    file_format: &FileFormat {
        id: 120_155_765,
        source_type: SourceType::Iana,
        name: "vnd.familysearch.gedcom+zip",
        extensions: &[],
        media_types: &["application/vnd.familysearch.gedcom+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
