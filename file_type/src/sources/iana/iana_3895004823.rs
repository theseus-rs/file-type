use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3895004823: FileFormat = FileFormat {
    id: 3_895_004_823,
    source_type: SourceType::Iana,
    name: "AMR-WB",
    extensions: &[],
    media_types: &["audio/AMR-WB"],
    internal_signatures: &[],
    related_formats: &[],
};
