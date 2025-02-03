use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_863547179: FileFormat = FileFormat {
    id: 863_547_179,
    source_type: SourceType::Iana,
    name: "xml-dtd",
    extensions: &[],
    media_types: &["application/xml-dtd"],
    internal_signatures: &[],
    related_formats: &[],
};
