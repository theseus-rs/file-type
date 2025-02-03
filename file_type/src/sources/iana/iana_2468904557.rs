use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2468904557: FileFormat = FileFormat {
    id: 2_468_904_557,
    source_type: SourceType::Iana,
    name: "vnd.radisys.moml+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.moml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
