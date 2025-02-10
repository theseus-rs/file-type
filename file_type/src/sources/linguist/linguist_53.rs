use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_53: FileType = FileType {
    file_format: &FileFormat {
        id: 53,
        source_type: SourceType::Linguist,
        name: "CartoCSS",
        extensions: &["mss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
