use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3537351692: FileFormat = FileFormat {
    id: 3_537_351_692,
    source_type: SourceType::Iana,
    name: "vnd.dece.mobile",
    extensions: &[],
    media_types: &["video/vnd.dece.mobile"],
    internal_signatures: &[],
    related_formats: &[],
};
