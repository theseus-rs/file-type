use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1283475108: FileFormat = FileFormat {
    id: 1_283_475_108,
    source_type: SourceType::Iana,
    name: "prs.alvestrand.titrax-sheet",
    extensions: &[],
    media_types: &["application/prs.alvestrand.titrax-sheet"],
    signatures: &[],
    related_formats: &[],
};
