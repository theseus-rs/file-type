use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_827296418: FileFormat = FileFormat {
    id: 827_296_418,
    source_type: SourceType::Iana,
    name: "jxl",
    extensions: &[],
    media_types: &["image/jxl"],
    internal_signatures: &[],
    related_formats: &[],
};
