use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1720057282: FileFormat = FileFormat {
    id: 1_720_057_282,
    source_type: SourceType::Iana,
    name: "vnd.gentics.grd+json",
    extensions: &[],
    media_types: &["application/vnd.gentics.grd+json"],
    internal_signatures: &[],
    related_formats: &[],
};
