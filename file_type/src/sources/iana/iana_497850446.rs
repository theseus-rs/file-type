use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_497850446: FileFormat = FileFormat {
    id: 497_850_446,
    source_type: SourceType::Iana,
    name: "prs.implied-document+xml",
    extensions: &[],
    media_types: &["application/prs.implied-document+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
