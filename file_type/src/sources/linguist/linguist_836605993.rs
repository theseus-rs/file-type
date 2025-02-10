use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_836605993: FileType = FileType {
    file_format: &FileFormat {
        id: 836_605_993,
        source_type: SourceType::Linguist,
        name: "WGSL",
        extensions: &["wgsl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
