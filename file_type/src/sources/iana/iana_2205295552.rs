use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2205295552: FileFormat = FileFormat {
    id: 2_205_295_552,
    source_type: SourceType::Iana,
    name: "vnd.dvb.ipdcesgaccess",
    extensions: &[],
    media_types: &["application/vnd.dvb.ipdcesgaccess"],
    internal_signatures: &[],
    related_formats: &[],
};
