use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3820781765: FileFormat = FileFormat {
    id: 3_820_781_765,
    source_type: SourceType::Iana,
    name: "shex",
    extensions: &[],
    media_types: &["text/shex"],
    internal_signatures: &[],
    related_formats: &[],
};
