use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3796410060: FileFormat = FileFormat {
    id: 3_796_410_060,
    source_type: SourceType::Iana,
    name: "vnd.pwg-multiplexed",
    extensions: &[],
    media_types: &["application/vnd.pwg-multiplexed"],
    internal_signatures: &[],
    related_formats: &[],
};
