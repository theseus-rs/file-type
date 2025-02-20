use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
