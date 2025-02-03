use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1648625350: FileFormat = FileFormat {
    id: 1_648_625_350,
    source_type: SourceType::Iana,
    name: "vnd.sailingtracker.track",
    extensions: &[],
    media_types: &["application/vnd.sailingtracker.track"],
    internal_signatures: &[],
    related_formats: &[],
};
