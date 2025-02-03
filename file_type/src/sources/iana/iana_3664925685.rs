use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3664925685: FileFormat = FileFormat {
    id: 3_664_925_685,
    source_type: SourceType::Iana,
    name: "vnd.espass-espass+zip",
    extensions: &[],
    media_types: &["application/vnd.espass-espass+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
