use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_886513491: FileFormat = FileFormat {
    id: 886_513_491,
    source_type: SourceType::Iana,
    name: "emma+xml",
    extensions: &[],
    media_types: &["application/emma+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
