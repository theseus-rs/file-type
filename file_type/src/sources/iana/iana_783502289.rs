use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_783502289: FileFormat = FileFormat {
    id: 783_502_289,
    source_type: SourceType::Iana,
    name: "index.cmd",
    extensions: &[],
    media_types: &["application/index.cmd"],
    internal_signatures: &[],
    related_formats: &[],
};
