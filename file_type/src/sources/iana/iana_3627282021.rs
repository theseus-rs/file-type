use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3627282021: FileFormat = FileFormat {
    id: 3_627_282_021,
    source_type: SourceType::Iana,
    name: "vnd.dvb.dvbj",
    extensions: &[],
    media_types: &["application/vnd.dvb.dvbj"],
    internal_signatures: &[],
    related_formats: &[],
};
