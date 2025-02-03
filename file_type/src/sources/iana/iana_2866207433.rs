use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2866207433: FileFormat = FileFormat {
    id: 2_866_207_433,
    source_type: SourceType::Iana,
    name: "vnd.mediastation.cdkey",
    extensions: &[],
    media_types: &["application/vnd.mediastation.cdkey"],
    internal_signatures: &[],
    related_formats: &[],
};
