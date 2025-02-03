use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1314571476: FileFormat = FileFormat {
    id: 1_314_571_476,
    source_type: SourceType::Iana,
    name: "vnd.nokia.mp4vr",
    extensions: &[],
    media_types: &["video/vnd.nokia.mp4vr"],
    internal_signatures: &[],
    related_formats: &[],
};
