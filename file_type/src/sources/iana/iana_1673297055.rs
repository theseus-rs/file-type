use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1673297055: FileFormat = FileFormat {
    id: 1_673_297_055,
    source_type: SourceType::Iana,
    name: "tiff-fx",
    extensions: &[],
    media_types: &["image/tiff-fx"],
    internal_signatures: &[],
    related_formats: &[],
};
