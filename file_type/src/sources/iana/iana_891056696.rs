use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_891056696: FileFormat = FileFormat {
    id: 891_056_696,
    source_type: SourceType::Iana,
    name: "vnd.MFER",
    extensions: &[],
    media_types: &["application/vnd.MFER"],
    internal_signatures: &[],
    related_formats: &[],
};
