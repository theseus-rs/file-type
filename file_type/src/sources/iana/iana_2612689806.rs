use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2612689806: FileFormat = FileFormat {
    id: 2_612_689_806,
    source_type: SourceType::Iana,
    name: "vnd.adobe.partial-upload",
    extensions: &[],
    media_types: &["application/vnd.adobe.partial-upload"],
    internal_signatures: &[],
    related_formats: &[],
};
