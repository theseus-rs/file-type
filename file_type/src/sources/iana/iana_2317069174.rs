use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2317069174: FileFormat = FileFormat {
    id: 2_317_069_174,
    source_type: SourceType::Iana,
    name: "CSTAdata+xml",
    extensions: &[],
    media_types: &["application/CSTAdata+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
