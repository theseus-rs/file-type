use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2189485322: FileFormat = FileFormat {
    id: 2_189_485_322,
    source_type: SourceType::Iana,
    name: "vnd.octel.sbc",
    extensions: &[],
    media_types: &["audio/vnd.octel.sbc"],
    internal_signatures: &[],
    related_formats: &[],
};
