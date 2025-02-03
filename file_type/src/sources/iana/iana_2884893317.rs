use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2884893317: FileFormat = FileFormat {
    id: 2_884_893_317,
    source_type: SourceType::Iana,
    name: "vnd.motorola.iprm",
    extensions: &[],
    media_types: &["application/vnd.motorola.iprm"],
    internal_signatures: &[],
    related_formats: &[],
};
