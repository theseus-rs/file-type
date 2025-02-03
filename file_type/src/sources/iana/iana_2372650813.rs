use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2372650813: FileFormat = FileFormat {
    id: 2_372_650_813,
    source_type: SourceType::Iana,
    name: "cdmi-capability",
    extensions: &[],
    media_types: &["application/cdmi-capability"],
    internal_signatures: &[],
    related_formats: &[],
};
