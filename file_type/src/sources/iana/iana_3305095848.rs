use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3305095848: FileFormat = FileFormat {
    id: 3_305_095_848,
    source_type: SourceType::Iana,
    name: "passport",
    extensions: &[],
    media_types: &["application/passport"],
    internal_signatures: &[],
    related_formats: &[],
};
