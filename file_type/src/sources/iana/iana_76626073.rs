use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_76626073: FileFormat = FileFormat {
    id: 76_626_073,
    source_type: SourceType::Iana,
    name: "rpki-updown",
    extensions: &[],
    media_types: &["application/rpki-updown"],
    internal_signatures: &[],
    related_formats: &[],
};
