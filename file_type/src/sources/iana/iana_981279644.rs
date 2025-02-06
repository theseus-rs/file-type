use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_981279644: FileFormat = FileFormat {
    id: 981_279_644,
    source_type: SourceType::Iana,
    name: "vnd.dvb.ait",
    extensions: &[],
    media_types: &["application/vnd.dvb.ait"],
    signatures: &[],
    related_formats: &[],
};
