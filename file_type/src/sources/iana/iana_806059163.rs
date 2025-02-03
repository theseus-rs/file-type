use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_806059163: FileFormat = FileFormat {
    id: 806_059_163,
    source_type: SourceType::Iana,
    name: "vnd.sealedmedia.softseal.gif",
    extensions: &[],
    media_types: &["image/vnd.sealedmedia.softseal.gif"],
    internal_signatures: &[],
    related_formats: &[],
};
