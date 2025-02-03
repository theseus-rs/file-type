use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3232900560: FileFormat = FileFormat {
    id: 3_232_900_560,
    source_type: SourceType::Iana,
    name: "TETRA_ISI",
    extensions: &[],
    media_types: &["application/TETRA_ISI"],
    internal_signatures: &[],
    related_formats: &[],
};
