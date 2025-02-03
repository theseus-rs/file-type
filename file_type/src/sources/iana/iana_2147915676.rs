use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2147915676: FileFormat = FileFormat {
    id: 2_147_915_676,
    source_type: SourceType::Iana,
    name: "vnd.xmpie.xlim",
    extensions: &[],
    media_types: &["application/vnd.xmpie.xlim"],
    internal_signatures: &[],
    related_formats: &[],
};
