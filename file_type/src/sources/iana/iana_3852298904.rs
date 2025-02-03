use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3852298904: FileFormat = FileFormat {
    id: 3_852_298_904,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-audit-conf+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-audit-conf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
