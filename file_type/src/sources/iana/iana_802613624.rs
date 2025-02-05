use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_802613624: FileFormat = FileFormat {
    id: 802_613_624,
    source_type: SourceType::Iana,
    name: "EVRCWB",
    extensions: &[],
    media_types: &["audio/EVRCWB"],
    signatures: &[],
    related_formats: &[],
};
