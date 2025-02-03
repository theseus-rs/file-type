use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2421251805: FileFormat = FileFormat {
    id: 2_421_251_805,
    source_type: SourceType::Iana,
    name: "aif+json",
    extensions: &[],
    media_types: &["application/aif+json"],
    internal_signatures: &[],
    related_formats: &[],
};
