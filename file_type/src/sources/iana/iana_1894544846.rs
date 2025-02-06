use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1894544846: FileFormat = FileFormat {
    id: 1_894_544_846,
    source_type: SourceType::Iana,
    name: "vnd.sun.wadl+xml",
    extensions: &[],
    media_types: &["application/vnd.sun.wadl+xml"],
    signatures: &[],
    related_formats: &[],
};
