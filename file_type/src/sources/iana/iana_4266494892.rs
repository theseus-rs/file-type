use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4266494892: FileFormat = FileFormat {
    id: 4_266_494_892,
    source_type: SourceType::Iana,
    name: "rls-services+xml",
    extensions: &[],
    media_types: &["application/rls-services+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
