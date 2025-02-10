use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_638334590: FileType = FileType {
    file_format: &FileFormat {
        id: 638_334_590,
        source_type: SourceType::Linguist,
        name: "Mustache",
        extensions: &["mustache"],
        media_types: &["text/x-smarty"],
        signatures: &[],
        related_formats: &[],
    },
};
