use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_692635484: FileType = FileType {
    file_format: &FileFormat {
        id: 692_635_484,
        source_type: SourceType::Linguist,
        name: "Kickstart",
        extensions: &["ks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
