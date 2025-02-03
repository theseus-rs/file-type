use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2445824637: FileFormat = FileFormat {
    id: 2_445_824_637,
    source_type: SourceType::Iana,
    name: "vnd.parasolid.transmit.binary",
    extensions: &[],
    media_types: &["model/vnd.parasolid.transmit.binary"],
    internal_signatures: &[],
    related_formats: &[],
};
