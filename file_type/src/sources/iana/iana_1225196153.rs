use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1225196153: FileFormat = FileFormat {
    id: 1_225_196_153,
    source_type: SourceType::Iana,
    name: "vnd.globalgraphics.pgb",
    extensions: &[],
    media_types: &["image/vnd.globalgraphics.pgb"],
    internal_signatures: &[],
    related_formats: &[],
};
