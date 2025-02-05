use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2802590849: FileFormat = FileFormat {
    id: 2_802_590_849,
    source_type: SourceType::Iana,
    name: "mpeg4-generic",
    extensions: &[],
    media_types: &["video/mpeg4-generic"],
    signatures: &[],
    related_formats: &[],
};
