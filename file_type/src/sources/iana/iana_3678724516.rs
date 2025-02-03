use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3678724516: FileFormat = FileFormat {
    id: 3_678_724_516,
    source_type: SourceType::Iana,
    name: "vnd.dvb.file",
    extensions: &[],
    media_types: &["video/vnd.dvb.file"],
    internal_signatures: &[],
    related_formats: &[],
};
