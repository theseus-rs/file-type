use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_541997281: FileFormat = FileFormat {
    id: 541_997_281,
    source_type: SourceType::Iana,
    name: "vnd.opentimestamps.ots",
    extensions: &[],
    media_types: &["application/vnd.opentimestamps.ots"],
    internal_signatures: &[],
    related_formats: &[],
};
