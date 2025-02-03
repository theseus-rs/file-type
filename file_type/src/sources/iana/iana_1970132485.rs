use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1970132485: FileFormat = FileFormat {
    id: 1_970_132_485,
    source_type: SourceType::Iana,
    name: "rtx",
    extensions: &[],
    media_types: &["video/rtx"],
    internal_signatures: &[],
    related_formats: &[],
};
