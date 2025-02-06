use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2018760936: FileFormat = FileFormat {
    id: 2_018_760_936,
    source_type: SourceType::Iana,
    name: "sipfrag",
    extensions: &[],
    media_types: &["message/sipfrag"],
    signatures: &[],
    related_formats: &[],
};
