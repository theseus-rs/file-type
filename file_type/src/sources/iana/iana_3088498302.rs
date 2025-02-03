use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3088498302: FileFormat = FileFormat {
    id: 3_088_498_302,
    source_type: SourceType::Iana,
    name: "L20",
    extensions: &[],
    media_types: &["audio/L20"],
    internal_signatures: &[],
    related_formats: &[],
};
