use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_53513821: FileFormat = FileFormat {
    id: 53_513_821,
    source_type: SourceType::Iana,
    name: "cose-x509",
    extensions: &[],
    media_types: &["application/cose-x509"],
    internal_signatures: &[],
    related_formats: &[],
};
