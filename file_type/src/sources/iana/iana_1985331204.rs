use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1985331204: FileFormat = FileFormat {
    id: 1_985_331_204,
    source_type: SourceType::Iana,
    name: "vnd.novadigm.EDX",
    extensions: &[],
    media_types: &["application/vnd.novadigm.EDX"],
    internal_signatures: &[],
    related_formats: &[],
};
