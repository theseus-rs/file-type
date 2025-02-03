use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1626546553: FileFormat = FileFormat {
    id: 1_626_546_553,
    source_type: SourceType::Iana,
    name: "vnd.isac.fcs",
    extensions: &[],
    media_types: &["application/vnd.isac.fcs"],
    internal_signatures: &[],
    related_formats: &[],
};
