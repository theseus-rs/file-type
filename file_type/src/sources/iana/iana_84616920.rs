use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_84616920: FileFormat = FileFormat {
    id: 84_616_920,
    source_type: SourceType::Iana,
    name: "EVRCB",
    extensions: &[],
    media_types: &["audio/EVRCB"],
    internal_signatures: &[],
    related_formats: &[],
};
