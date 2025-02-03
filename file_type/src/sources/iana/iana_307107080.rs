use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_307107080: FileFormat = FileFormat {
    id: 307_107_080,
    source_type: SourceType::Iana,
    name: "prs.sid",
    extensions: &[],
    media_types: &["audio/prs.sid"],
    internal_signatures: &[],
    related_formats: &[],
};
