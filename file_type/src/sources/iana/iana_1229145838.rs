use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1229145838: FileFormat = FileFormat {
    id: 1_229_145_838,
    source_type: SourceType::Iana,
    name: "vnd.amiga.ami",
    extensions: &[],
    media_types: &["application/vnd.amiga.ami"],
    internal_signatures: &[],
    related_formats: &[],
};
