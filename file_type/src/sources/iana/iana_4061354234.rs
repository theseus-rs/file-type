use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4061354234: FileFormat = FileFormat {
    id: 4_061_354_234,
    source_type: SourceType::Iana,
    name: "vnd.sybyl.mol2",
    extensions: &[],
    media_types: &["application/vnd.sybyl.mol2"],
    internal_signatures: &[],
    related_formats: &[],
};
