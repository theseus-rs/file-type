use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2837889784: FileFormat = FileFormat {
    id: 2_837_889_784,
    source_type: SourceType::Iana,
    name: "vnd.djvu",
    extensions: &[],
    media_types: &["image/vnd.djvu"],
    internal_signatures: &[],
    related_formats: &[],
};
