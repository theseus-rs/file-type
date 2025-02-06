use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_510240730: FileFormat = FileFormat {
    id: 510_240_730,
    source_type: SourceType::Iana,
    name: "vnd.dvb.file",
    extensions: &[],
    media_types: &["audio/vnd.dvb.file"],
    signatures: &[],
    related_formats: &[],
};
