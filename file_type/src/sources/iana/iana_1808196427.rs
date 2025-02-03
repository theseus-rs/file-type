use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1808196427: FileFormat = FileFormat {
    id: 1_808_196_427,
    source_type: SourceType::Iana,
    name: "vnd.keyman.kmx",
    extensions: &[],
    media_types: &["application/vnd.keyman.kmx"],
    internal_signatures: &[],
    related_formats: &[],
};
