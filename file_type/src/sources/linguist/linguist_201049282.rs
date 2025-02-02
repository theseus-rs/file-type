use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_201049282: FileFormat = FileFormat {
    id: 201_049_282,
    source_type: SourceType::Linguist,
    name: "DirectX 3D File",
    extensions: &["x"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
