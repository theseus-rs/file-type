use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4153744506: FileFormat = FileFormat {
    id: 4_153_744_506,
    source_type: SourceType::Iana,
    name: "vnd.dvb.ipdcroaming",
    extensions: &[],
    media_types: &["application/vnd.dvb.ipdcroaming"],
    signatures: &[],
    related_formats: &[],
};
