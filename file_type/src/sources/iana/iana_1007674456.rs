use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1007674456: FileFormat = FileFormat {
    id: 1_007_674_456,
    source_type: SourceType::Iana,
    name: "vnd.sss-cod",
    extensions: &[],
    media_types: &["application/vnd.sss-cod"],
    internal_signatures: &[],
    related_formats: &[],
};
