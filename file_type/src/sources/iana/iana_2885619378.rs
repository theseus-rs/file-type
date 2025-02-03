use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2885619378: FileFormat = FileFormat {
    id: 2_885_619_378,
    source_type: SourceType::Iana,
    name: "vnd.youtube.yt (OBSOLETED in favor of video/vnd.youtube.yt)",
    extensions: &[],
    media_types: &["application/vnd.youtube.yt"],
    internal_signatures: &[],
    related_formats: &[],
};
