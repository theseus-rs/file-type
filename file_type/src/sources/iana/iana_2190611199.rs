use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2190611199: FileFormat = FileFormat {
    id: 2_190_611_199,
    source_type: SourceType::Iana,
    name: "vnd.intu.qbo",
    extensions: &[],
    media_types: &["application/vnd.intu.qbo"],
    internal_signatures: &[],
    related_formats: &[],
};
