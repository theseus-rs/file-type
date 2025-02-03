use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_692040152: FileFormat = FileFormat {
    id: 692_040_152,
    source_type: SourceType::Iana,
    name: "vnd.svf",
    extensions: &[],
    media_types: &["image/vnd.svf"],
    internal_signatures: &[],
    related_formats: &[],
};
