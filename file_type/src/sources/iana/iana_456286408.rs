use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_456286408: FileFormat = FileFormat {
    id: 456_286_408,
    source_type: SourceType::Iana,
    name: "route-s-tsid+xml",
    extensions: &[],
    media_types: &["application/route-s-tsid+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
