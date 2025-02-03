use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3859408122: FileFormat = FileFormat {
    id: 3_859_408_122,
    source_type: SourceType::Iana,
    name: "vnd.cups-postscript",
    extensions: &[],
    media_types: &["application/vnd.cups-postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
