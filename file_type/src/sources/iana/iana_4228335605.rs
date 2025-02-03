use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4228335605: FileFormat = FileFormat {
    id: 4_228_335_605,
    source_type: SourceType::Iana,
    name: "vnd.osgi.dp",
    extensions: &[],
    media_types: &["application/vnd.osgi.dp"],
    internal_signatures: &[],
    related_formats: &[],
};
