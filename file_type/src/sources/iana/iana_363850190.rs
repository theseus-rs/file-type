use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_363850190: FileFormat = FileFormat {
    id: 363_850_190,
    source_type: SourceType::Iana,
    name: "vnd.sealedmedia.softseal.jpg",
    extensions: &[],
    media_types: &["image/vnd.sealedmedia.softseal.jpg"],
    signatures: &[],
    related_formats: &[],
};
