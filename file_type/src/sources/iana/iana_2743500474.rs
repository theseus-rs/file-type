use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2743500474: FileFormat = FileFormat {
    id: 2_743_500_474,
    source_type: SourceType::Iana,
    name: "vnd.maxar.archive.3tz+zip",
    extensions: &[],
    media_types: &["application/vnd.maxar.archive.3tz+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
