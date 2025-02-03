use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1046049563: FileFormat = FileFormat {
    id: 1_046_049_563,
    source_type: SourceType::Iana,
    name: "vnd.3lightssoftware.imagescal",
    extensions: &[],
    media_types: &["application/vnd.3lightssoftware.imagescal"],
    internal_signatures: &[],
    related_formats: &[],
};
