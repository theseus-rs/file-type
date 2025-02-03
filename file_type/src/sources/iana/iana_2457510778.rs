use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2457510778: FileFormat = FileFormat {
    id: 2_457_510_778,
    source_type: SourceType::Iana,
    name: "prs.implied-object+json-seq",
    extensions: &[],
    media_types: &["application/prs.implied-object+json-seq"],
    internal_signatures: &[],
    related_formats: &[],
};
