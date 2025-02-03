use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3112925738: FileFormat = FileFormat {
    id: 3_112_925_738,
    source_type: SourceType::Iana,
    name: "lgr+xml",
    extensions: &[],
    media_types: &["application/lgr+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
