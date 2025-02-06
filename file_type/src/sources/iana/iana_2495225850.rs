use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2495225850: FileFormat = FileFormat {
    id: 2_495_225_850,
    source_type: SourceType::Iana,
    name: "H264",
    extensions: &[],
    media_types: &["video/H264"],
    signatures: &[],
    related_formats: &[],
};
