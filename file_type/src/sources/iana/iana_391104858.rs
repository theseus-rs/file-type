use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_391104858: FileFormat = FileFormat {
    id: 391_104_858,
    source_type: SourceType::Iana,
    name: "reginfo+xml",
    extensions: &[],
    media_types: &["application/reginfo+xml"],
    signatures: &[],
    related_formats: &[],
};
