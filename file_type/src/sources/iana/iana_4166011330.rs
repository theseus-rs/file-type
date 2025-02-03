use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4166011330: FileFormat = FileFormat {
    id: 4_166_011_330,
    source_type: SourceType::Iana,
    name: "rtp-enc-aescm128",
    extensions: &[],
    media_types: &["audio/rtp-enc-aescm128"],
    internal_signatures: &[],
    related_formats: &[],
};
