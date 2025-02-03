use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1084097919: FileFormat = FileFormat {
    id: 1_084_097_919,
    source_type: SourceType::Iana,
    name: "vnd.wrq-hp3000-labelled",
    extensions: &[],
    media_types: &["application/vnd.wrq-hp3000-labelled"],
    internal_signatures: &[],
    related_formats: &[],
};
