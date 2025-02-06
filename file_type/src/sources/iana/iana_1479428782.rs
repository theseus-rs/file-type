use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1479428782: FileFormat = FileFormat {
    id: 1_479_428_782,
    source_type: SourceType::Iana,
    name: "vnd.dvb.iptv.alfec-enhancement",
    extensions: &[],
    media_types: &["application/vnd.dvb.iptv.alfec-enhancement"],
    signatures: &[],
    related_formats: &[],
};
