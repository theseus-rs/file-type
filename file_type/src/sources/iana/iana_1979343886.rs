use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1979343886: FileFormat = FileFormat {
    id: 1_979_343_886,
    source_type: SourceType::Iana,
    name: "mp4",
    extensions: &[],
    media_types: &["application/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
