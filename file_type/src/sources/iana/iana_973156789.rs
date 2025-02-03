use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_973156789: FileFormat = FileFormat {
    id: 973_156_789,
    source_type: SourceType::Iana,
    name: "vnd.Mobius.MSL",
    extensions: &[],
    media_types: &["application/vnd.Mobius.MSL"],
    internal_signatures: &[],
    related_formats: &[],
};
