use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_863358936: FileFormat = FileFormat {
    id: 863_358_936,
    source_type: SourceType::Iana,
    name: "cnrp+xml",
    extensions: &[],
    media_types: &["application/cnrp+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
