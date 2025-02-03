use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1220007935: FileFormat = FileFormat {
    id: 1_220_007_935,
    source_type: SourceType::Iana,
    name: "richtext",
    extensions: &[],
    media_types: &["text/richtext"],
    internal_signatures: &[],
    related_formats: &[],
};
