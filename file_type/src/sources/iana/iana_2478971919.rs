use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2478971919: FileFormat = FileFormat {
    id: 2_478_971_919,
    source_type: SourceType::Iana,
    name: "vnd.sealedmedia.softseal.mov",
    extensions: &[],
    media_types: &["video/vnd.sealedmedia.softseal.mov"],
    internal_signatures: &[],
    related_formats: &[],
};
