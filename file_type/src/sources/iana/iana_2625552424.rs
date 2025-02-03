use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2625552424: FileFormat = FileFormat {
    id: 2_625_552_424,
    source_type: SourceType::Iana,
    name: "vnd.wolfram.player",
    extensions: &[],
    media_types: &["application/vnd.wolfram.player"],
    internal_signatures: &[],
    related_formats: &[],
};
