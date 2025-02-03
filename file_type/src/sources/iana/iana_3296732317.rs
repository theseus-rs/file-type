use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3296732317: FileFormat = FileFormat {
    id: 3_296_732_317,
    source_type: SourceType::Iana,
    name: "spdx",
    extensions: &[],
    media_types: &["text/spdx"],
    internal_signatures: &[],
    related_formats: &[],
};
