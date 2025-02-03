use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2803451356: FileFormat = FileFormat {
    id: 2_803_451_356,
    source_type: SourceType::Iana,
    name: "ATFX",
    extensions: &[],
    media_types: &["application/ATFX"],
    internal_signatures: &[],
    related_formats: &[],
};
