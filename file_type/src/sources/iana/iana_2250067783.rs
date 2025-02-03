use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2250067783: FileFormat = FileFormat {
    id: 2_250_067_783,
    source_type: SourceType::Iana,
    name: "vnd.oipf.dae.svg+xml",
    extensions: &[],
    media_types: &["application/vnd.oipf.dae.svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
