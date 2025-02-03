use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2173042511: FileFormat = FileFormat {
    id: 2_173_042_511,
    source_type: SourceType::Iana,
    name: "rpki-checklist",
    extensions: &[],
    media_types: &["application/rpki-checklist"],
    internal_signatures: &[],
    related_formats: &[],
};
