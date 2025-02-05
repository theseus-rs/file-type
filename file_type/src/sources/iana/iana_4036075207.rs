use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4036075207: FileFormat = FileFormat {
    id: 4_036_075_207,
    source_type: SourceType::Iana,
    name: "vnd.vsf",
    extensions: &[],
    media_types: &["application/vnd.vsf"],
    signatures: &[],
    related_formats: &[],
};
