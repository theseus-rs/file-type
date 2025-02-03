use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2981225743: FileFormat = FileFormat {
    id: 2_981_225_743,
    source_type: SourceType::Iana,
    name: "tamp-apex-update-confirm",
    extensions: &[],
    media_types: &["application/tamp-apex-update-confirm"],
    internal_signatures: &[],
    related_formats: &[],
};
