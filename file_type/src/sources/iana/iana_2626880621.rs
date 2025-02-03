use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2626880621: FileFormat = FileFormat {
    id: 2_626_880_621,
    source_type: SourceType::Iana,
    name: "vnd.fluxtime.clip",
    extensions: &[],
    media_types: &["application/vnd.fluxtime.clip"],
    internal_signatures: &[],
    related_formats: &[],
};
