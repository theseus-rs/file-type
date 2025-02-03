use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3785618288: FileFormat = FileFormat {
    id: 3_785_618_288,
    source_type: SourceType::Iana,
    name: "xmpp+xml",
    extensions: &[],
    media_types: &["application/xmpp+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
