use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_482286266: FileFormat = FileFormat {
    id: 482_286_266,
    source_type: SourceType::Iana,
    name: "marcxml+xml",
    extensions: &[],
    media_types: &["application/marcxml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
