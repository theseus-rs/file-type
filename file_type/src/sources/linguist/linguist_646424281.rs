use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_646424281: FileType = FileType {
    file_format: &FileFormat {
        id: 646_424_281,
        source_type: SourceType::Linguist,
        name: "Zig",
        extensions: &["zig", "zig.zon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
