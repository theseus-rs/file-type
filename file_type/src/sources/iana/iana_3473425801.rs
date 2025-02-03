use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3473425801: FileFormat = FileFormat {
    id: 3_473_425_801,
    source_type: SourceType::Iana,
    name: "vnd.clonk.c4group",
    extensions: &[],
    media_types: &["application/vnd.clonk.c4group"],
    internal_signatures: &[],
    related_formats: &[],
};
