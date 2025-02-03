use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1236131182: FileFormat = FileFormat {
    id: 1_236_131_182,
    source_type: SourceType::Iana,
    name: "vnd.joost.joda-archive",
    extensions: &[],
    media_types: &["application/vnd.joost.joda-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
