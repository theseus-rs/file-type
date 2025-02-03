use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3318388259: FileFormat = FileFormat {
    id: 3_318_388_259,
    source_type: SourceType::Iana,
    name: "rtploopback",
    extensions: &[],
    media_types: &["text/rtploopback"],
    internal_signatures: &[],
    related_formats: &[],
};
