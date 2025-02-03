use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4180165785: FileFormat = FileFormat {
    id: 4_180_165_785,
    source_type: SourceType::Iana,
    name: "mobile-xmf",
    extensions: &[],
    media_types: &["audio/mobile-xmf"],
    internal_signatures: &[],
    related_formats: &[],
};
