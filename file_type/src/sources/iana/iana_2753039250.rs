use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2753039250: FileFormat = FileFormat {
    id: 2_753_039_250,
    source_type: SourceType::Iana,
    name: "vnd.youtube.yt",
    extensions: &[],
    media_types: &["video/vnd.youtube.yt"],
    signatures: &[],
    related_formats: &[],
};
