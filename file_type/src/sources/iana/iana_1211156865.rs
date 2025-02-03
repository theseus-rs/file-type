use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1211156865: FileFormat = FileFormat {
    id: 1_211_156_865,
    source_type: SourceType::Iana,
    name: "vnd.miele+json",
    extensions: &[],
    media_types: &["application/vnd.miele+json"],
    internal_signatures: &[],
    related_formats: &[],
};
