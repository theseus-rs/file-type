use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2796510911: FileFormat = FileFormat {
    id: 2_796_510_911,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.image",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.image"],
    internal_signatures: &[],
    related_formats: &[],
};
