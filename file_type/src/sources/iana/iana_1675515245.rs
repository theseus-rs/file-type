use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1675515245: FileFormat = FileFormat {
    id: 1_675_515_245,
    source_type: SourceType::Iana,
    name: "vnd.blueice.multipass",
    extensions: &[],
    media_types: &["application/vnd.blueice.multipass"],
    internal_signatures: &[],
    related_formats: &[],
};
