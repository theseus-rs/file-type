use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_327: FileType = FileType {
    file_format: &FileFormat {
        id: 327,
        source_type: SourceType::Linguist,
        name: "Rust",
        extensions: &["rs", "rs.in"],
        media_types: &["text/x-rustsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
