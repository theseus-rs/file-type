use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2059809139: FileFormat = FileFormat {
    id: 2_059_809_139,
    source_type: SourceType::Iana,
    name: "vnd.adobe.flash.movie",
    extensions: &[],
    media_types: &["application/vnd.adobe.flash.movie"],
    signatures: &[],
    related_formats: &[],
};
