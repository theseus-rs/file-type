use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2734779849: FileFormat = FileFormat {
    id: 2_734_779_849,
    source_type: SourceType::Iana,
    name: "vnd.gtw",
    extensions: &[],
    media_types: &["model/vnd.gtw"],
    internal_signatures: &[],
    related_formats: &[],
};
