use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_816441770: FileFormat = FileFormat {
    id: 816_441_770,
    source_type: SourceType::Iana,
    name: "rtp-enc-aescm128",
    extensions: &[],
    media_types: &["video/rtp-enc-aescm128"],
    internal_signatures: &[],
    related_formats: &[],
};
