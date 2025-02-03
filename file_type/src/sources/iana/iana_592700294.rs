use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_592700294: FileFormat = FileFormat {
    id: 592_700_294,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-audit+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-audit+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
