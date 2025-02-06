use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_672006244: FileFormat = FileFormat {
    id: 672_006_244,
    source_type: SourceType::Iana,
    name: "prs.prop.logic",
    extensions: &[],
    media_types: &["text/prs.prop.logic"],
    signatures: &[],
    related_formats: &[],
};
