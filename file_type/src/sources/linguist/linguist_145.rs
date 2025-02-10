use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_145: FileType = FileType {
    file_format: &FileFormat {
        id: 145,
        source_type: SourceType::Linguist,
        name: "HLSL",
        extensions: &["cginc", "fx", "fxh", "hlsl", "hlsli"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
