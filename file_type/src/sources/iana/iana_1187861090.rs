use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1187861090: FileFormat = FileFormat {
    id: 1_187_861_090,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.lpp",
    extensions: &[],
    media_types: &["application/vnd.3gpp.lpp"],
    signatures: &[],
    related_formats: &[],
};
