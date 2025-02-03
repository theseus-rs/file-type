use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2643175794: FileFormat = FileFormat {
    id: 2_643_175_794,
    source_type: SourceType::Iana,
    name: "AV1",
    extensions: &[],
    media_types: &["video/AV1"],
    internal_signatures: &[],
    related_formats: &[],
};
