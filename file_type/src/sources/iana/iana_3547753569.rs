use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3547753569: FileFormat = FileFormat {
    id: 3_547_753_569,
    source_type: SourceType::Iana,
    name: "s-http (OBSOLETE)",
    extensions: &[],
    media_types: &["message/s-http"],
    internal_signatures: &[],
    related_formats: &[],
};
