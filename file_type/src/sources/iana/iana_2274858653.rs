use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2274858653: FileFormat = FileFormat {
    id: 2_274_858_653,
    source_type: SourceType::Iana,
    name: "vnd.dece.mp4",
    extensions: &[],
    media_types: &["video/vnd.dece.mp4"],
    signatures: &[],
    related_formats: &[],
};
