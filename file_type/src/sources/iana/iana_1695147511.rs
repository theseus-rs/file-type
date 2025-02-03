use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1695147511: FileFormat = FileFormat {
    id: 1_695_147_511,
    source_type: SourceType::Iana,
    name: "vnd.CELP",
    extensions: &[],
    media_types: &["audio/vnd.CELP"],
    internal_signatures: &[],
    related_formats: &[],
};
