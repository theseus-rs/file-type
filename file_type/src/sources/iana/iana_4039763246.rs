use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4039763246: FileFormat = FileFormat {
    id: 4_039_763_246,
    source_type: SourceType::Iana,
    name: "mrb-publish+xml",
    extensions: &[],
    media_types: &["application/mrb-publish+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
