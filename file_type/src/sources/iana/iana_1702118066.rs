use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1702118066: FileFormat = FileFormat {
    id: 1_702_118_066,
    source_type: SourceType::Iana,
    name: "vnd.gentoo.catmetadata+xml",
    extensions: &[],
    media_types: &["application/vnd.gentoo.catmetadata+xml"],
    signatures: &[],
    related_formats: &[],
};
