use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_295: FileType = FileType {
    file_format: &FileFormat {
        id: 295,
        source_type: SourceType::Linguist,
        name: "Prolog",
        extensions: &["pl", "plt", "pro", "prolog", "yap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
