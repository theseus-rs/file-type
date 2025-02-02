use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_135: FileFormat = FileFormat {
    id: 135,
    source_type: SourceType::Linguist,
    name: "Grace",
    extensions: &["grace"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
