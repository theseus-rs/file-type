use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3086100026: FileFormat = FileFormat {
    id: 3_086_100_026,
    source_type: SourceType::Iana,
    name: "vnd.oxli.countgraph",
    extensions: &[],
    media_types: &["application/vnd.oxli.countgraph"],
    signatures: &[],
    related_formats: &[],
};
