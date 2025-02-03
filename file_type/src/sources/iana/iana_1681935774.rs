use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1681935774: FileFormat = FileFormat {
    id: 1_681_935_774,
    source_type: SourceType::Iana,
    name: "vnd.si.uricatalogue (OBSOLETED by request)",
    extensions: &[],
    media_types: &["text/vnd.si.uricatalogue"],
    internal_signatures: &[],
    related_formats: &[],
};
