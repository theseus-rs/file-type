use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3334640110: FileFormat = FileFormat {
    id: 3_334_640_110,
    source_type: SourceType::Iana,
    name: "vnd.chemdraw+xml",
    extensions: &[],
    media_types: &["application/vnd.chemdraw+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
