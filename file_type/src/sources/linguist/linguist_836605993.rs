use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_836605993: FileFormat = FileFormat {
    id: 836_605_993,
    source_type: SourceType::Linguist,
    name: "WGSL",
    extensions: &["wgsl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
