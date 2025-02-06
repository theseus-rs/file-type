use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3583731876: FileFormat = FileFormat {
    id: 3_583_731_876,
    source_type: SourceType::Iana,
    name: "vnd.nokia.n-gage.symbian.install (OBSOLETE; no replacement given)",
    extensions: &[],
    media_types: &["application/vnd.nokia.n-gage.symbian.install"],
    signatures: &[],
    related_formats: &[],
};
