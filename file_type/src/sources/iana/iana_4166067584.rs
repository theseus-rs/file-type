use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4166067584: FileFormat = FileFormat {
    id: 4_166_067_584,
    source_type: SourceType::Iana,
    name: "mediaservercontrol+xml",
    extensions: &[],
    media_types: &["application/mediaservercontrol+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
