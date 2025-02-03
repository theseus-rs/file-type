use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1258888362: FileFormat = FileFormat {
    id: 1_258_888_362,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-audit-stream+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-audit-stream+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
