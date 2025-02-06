use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_145: FileFormat = FileFormat {
    id: 145,
    source_type: SourceType::Linguist,
    name: "HLSL",
    extensions: &["cginc", "fx", "fxh", "hlsl", "hlsli"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
