use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4050793986: FileFormat = FileFormat {
    id: 4_050_793_986,
    source_type: SourceType::Iana,
    name: "vnd.smintio.portals.archive",
    extensions: &[],
    media_types: &["application/vnd.smintio.portals.archive"],
    signatures: &[],
    related_formats: &[],
};
