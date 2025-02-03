use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1724265822: FileFormat = FileFormat {
    id: 1_724_265_822,
    source_type: SourceType::Iana,
    name: "vnd.nokia.mobile-xmf",
    extensions: &[],
    media_types: &["audio/vnd.nokia.mobile-xmf"],
    internal_signatures: &[],
    related_formats: &[],
};
