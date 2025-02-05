use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_64343561: FileFormat = FileFormat {
    id: 64_343_561,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-audit-conn+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-audit-conn+xml"],
    signatures: &[],
    related_formats: &[],
};
