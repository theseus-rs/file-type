use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2349492241: FileFormat = FileFormat {
    id: 2_349_492_241,
    source_type: SourceType::Iana,
    name: "vnd.hans",
    extensions: &[],
    media_types: &["text/vnd.hans"],
    signatures: &[],
    related_formats: &[],
};
