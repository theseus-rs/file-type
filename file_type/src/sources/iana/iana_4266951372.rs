use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4266951372: FileFormat = FileFormat {
    id: 4_266_951_372,
    source_type: SourceType::Iana,
    name: "vnd.nokia.ncd",
    extensions: &[],
    media_types: &["application/vnd.nokia.ncd"],
    internal_signatures: &[],
    related_formats: &[],
};
