use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3297309654: FileFormat = FileFormat {
    id: 3_297_309_654,
    source_type: SourceType::Iana,
    name: "G729D",
    extensions: &[],
    media_types: &["audio/G729D"],
    internal_signatures: &[],
    related_formats: &[],
};
