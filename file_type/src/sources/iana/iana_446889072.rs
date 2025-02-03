use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_446889072: FileFormat = FileFormat {
    id: 446_889_072,
    source_type: SourceType::Iana,
    name: "dns",
    extensions: &[],
    media_types: &["application/dns"],
    internal_signatures: &[],
    related_formats: &[],
};
