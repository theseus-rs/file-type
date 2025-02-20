use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_271471144: FileType = FileType {
    file_format: &FileFormat {
        id: 271_471_144,
        source_type: SourceType::Linguist,
        name: "Sway",
        extensions: &["sw"],
        media_types: &["text/x-rustsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
