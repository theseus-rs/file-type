use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1802856897: FileFormat = FileFormat {
    id: 1_802_856_897,
    source_type: SourceType::Iana,
    name: "vnd.hcl-bireports",
    extensions: &[],
    media_types: &["application/vnd.hcl-bireports"],
    internal_signatures: &[],
    related_formats: &[],
};
