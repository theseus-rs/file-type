use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3784283486: FileFormat = FileFormat {
    id: 3_784_283_486,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["audio/example"],
    internal_signatures: &[],
    related_formats: &[],
};
