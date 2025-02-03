use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1066913277: FileFormat = FileFormat {
    id: 1_066_913_277,
    source_type: SourceType::Iana,
    name: "vnd.ncd.control",
    extensions: &[],
    media_types: &["application/vnd.ncd.control"],
    internal_signatures: &[],
    related_formats: &[],
};
