use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3769103005: FileFormat = FileFormat {
    id: 3_769_103_005,
    source_type: SourceType::Iana,
    name: "vnd.ms-wmdrm.meter-chlg-req",
    extensions: &[],
    media_types: &["application/vnd.ms-wmdrm.meter-chlg-req"],
    internal_signatures: &[],
    related_formats: &[],
};
