use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2233227456: FileFormat = FileFormat {
    id: 2_233_227_456,
    source_type: SourceType::Iana,
    name: "vnd.fdsn.mseed",
    extensions: &[],
    media_types: &["application/vnd.fdsn.mseed"],
    internal_signatures: &[],
    related_formats: &[],
};
