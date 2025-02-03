use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_734796685: FileFormat = FileFormat {
    id: 734_796_685,
    source_type: SourceType::Iana,
    name: "jscontact+json",
    extensions: &[],
    media_types: &["application/jscontact+json"],
    internal_signatures: &[],
    related_formats: &[],
};
