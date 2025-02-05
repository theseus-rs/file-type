use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2350179153: FileFormat = FileFormat {
    id: 2_350_179_153,
    source_type: SourceType::Iana,
    name: "jxs",
    extensions: &[],
    media_types: &["image/jxs"],
    signatures: &[],
    related_formats: &[],
};
