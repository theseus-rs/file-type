use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2888274652: FileFormat = FileFormat {
    id: 2_888_274_652,
    source_type: SourceType::Iana,
    name: "AMR",
    extensions: &[],
    media_types: &["audio/AMR"],
    internal_signatures: &[],
    related_formats: &[],
};
