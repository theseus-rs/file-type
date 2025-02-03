use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1244006042: FileFormat = FileFormat {
    id: 1_244_006_042,
    source_type: SourceType::Iana,
    name: "vnd.etsi.pstn+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.pstn+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
