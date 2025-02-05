use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2942086789: FileFormat = FileFormat {
    id: 2_942_086_789,
    source_type: SourceType::Iana,
    name: "prs.pti",
    extensions: &[],
    media_types: &["image/prs.pti"],
    signatures: &[],
    related_formats: &[],
};
