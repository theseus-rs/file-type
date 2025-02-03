use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3062216814: FileFormat = FileFormat {
    id: 3_062_216_814,
    source_type: SourceType::Iana,
    name: "prs.mayfile",
    extensions: &[],
    media_types: &["application/prs.mayfile"],
    internal_signatures: &[],
    related_formats: &[],
};
