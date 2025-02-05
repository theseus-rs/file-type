use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_972985342: FileFormat = FileFormat {
    id: 972_985_342,
    source_type: SourceType::Iana,
    name: "vnd.dvb.ipdcesgaccess2",
    extensions: &[],
    media_types: &["application/vnd.dvb.ipdcesgaccess2"],
    signatures: &[],
    related_formats: &[],
};
