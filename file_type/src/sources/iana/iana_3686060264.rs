use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3686060264: FileFormat = FileFormat {
    id: 3_686_060_264,
    source_type: SourceType::Iana,
    name: "vnd.ms-wmdrm.lic-chlg-req",
    extensions: &[],
    media_types: &["application/vnd.ms-wmdrm.lic-chlg-req"],
    signatures: &[],
    related_formats: &[],
};
