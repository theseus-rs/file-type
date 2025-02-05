use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2709085623: FileFormat = FileFormat {
    id: 2_709_085_623,
    source_type: SourceType::Iana,
    name: "vnd.dvb.pfr",
    extensions: &[],
    media_types: &["application/vnd.dvb.pfr"],
    signatures: &[],
    related_formats: &[],
};
