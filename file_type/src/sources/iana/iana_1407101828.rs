use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1407101828: FileFormat = FileFormat {
    id: 1_407_101_828,
    source_type: SourceType::Iana,
    name: "vnd.dece.data",
    extensions: &[],
    media_types: &["application/vnd.dece.data"],
    internal_signatures: &[],
    related_formats: &[],
};
