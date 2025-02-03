use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2917306439: FileFormat = FileFormat {
    id: 2_917_306_439,
    source_type: SourceType::Iana,
    name: "global-headers",
    extensions: &[],
    media_types: &["message/global-headers"],
    internal_signatures: &[],
    related_formats: &[],
};
