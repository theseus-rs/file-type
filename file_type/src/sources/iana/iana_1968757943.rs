use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1968757943: FileFormat = FileFormat {
    id: 1_968_757_943,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.sgdu",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.sgdu"],
    internal_signatures: &[],
    related_formats: &[],
};
