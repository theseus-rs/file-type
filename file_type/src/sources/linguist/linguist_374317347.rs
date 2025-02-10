use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_374317347: FileType = FileType {
    file_format: &FileFormat {
        id: 374_317_347,
        source_type: SourceType::Linguist,
        name: "OpenType Feature File",
        extensions: &["fea"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
