use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2220125132: FileFormat = FileFormat {
    id: 2_220_125_132,
    source_type: SourceType::Iana,
    name: "vnd.xmpie.plan",
    extensions: &[],
    media_types: &["application/vnd.xmpie.plan"],
    internal_signatures: &[],
    related_formats: &[],
};
