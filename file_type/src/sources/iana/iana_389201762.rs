use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_389201762: FileFormat = FileFormat {
    id: 389_201_762,
    source_type: SourceType::Iana,
    name: "vnd.xfdl.webform",
    extensions: &[],
    media_types: &["application/vnd.xfdl.webform"],
    internal_signatures: &[],
    related_formats: &[],
};
