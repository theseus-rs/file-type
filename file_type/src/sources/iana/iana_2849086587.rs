use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2849086587: FileFormat = FileFormat {
    id: 2_849_086_587,
    source_type: SourceType::Iana,
    name: "vnd.vd-study",
    extensions: &[],
    media_types: &["application/vnd.vd-study"],
    signatures: &[],
    related_formats: &[],
};
