use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_78756907: FileFormat = FileFormat {
    id: 78_756_907,
    source_type: SourceType::Iana,
    name: "vnd.gs-gdl",
    extensions: &[],
    media_types: &["model/vnd.gs-gdl"],
    signatures: &[],
    related_formats: &[],
};
