use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4207666045: FileFormat = FileFormat {
    id: 4_207_666_045,
    source_type: SourceType::Iana,
    name: "vnd.las.las+json",
    extensions: &[],
    media_types: &["application/vnd.las.las+json"],
    signatures: &[],
    related_formats: &[],
};
