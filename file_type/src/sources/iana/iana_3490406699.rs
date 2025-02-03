use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3490406699: FileFormat = FileFormat {
    id: 3_490_406_699,
    source_type: SourceType::Iana,
    name: "signed",
    extensions: &[],
    media_types: &["multipart/signed"],
    internal_signatures: &[],
    related_formats: &[],
};
