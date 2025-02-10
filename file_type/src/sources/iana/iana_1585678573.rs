use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1585678573: FileType = FileType {
    file_format: &FileFormat {
        id: 1_585_678_573,
        source_type: SourceType::Iana,
        name: "moss-keys",
        extensions: &[],
        media_types: &["application/moss-keys"],
        signatures: &[],
        related_formats: &[],
    },
};
