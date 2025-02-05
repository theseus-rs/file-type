use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_925464555: FileFormat = FileFormat {
    id: 925_464_555,
    source_type: SourceType::Iana,
    name: "vnd.rs-274x",
    extensions: &[],
    media_types: &["application/vnd.rs-274x"],
    signatures: &[],
    related_formats: &[],
};
