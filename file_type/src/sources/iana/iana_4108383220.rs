use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4108383220: FileFormat = FileFormat {
    id: 4_108_383_220,
    source_type: SourceType::Iana,
    name: "vnd.dtg.local.flash",
    extensions: &[],
    media_types: &["application/vnd.dtg.local.flash"],
    signatures: &[],
    related_formats: &[],
};
