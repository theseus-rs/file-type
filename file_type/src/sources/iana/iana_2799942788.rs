use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2799942788: FileFormat = FileFormat {
    id: 2_799_942_788,
    source_type: SourceType::Iana,
    name: "vnd.dvb.subtitle",
    extensions: &[],
    media_types: &["image/vnd.dvb.subtitle"],
    signatures: &[],
    related_formats: &[],
};
