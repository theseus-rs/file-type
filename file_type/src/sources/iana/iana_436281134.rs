use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_436281134: FileFormat = FileFormat {
    id: 436_281_134,
    source_type: SourceType::Iana,
    name: "vnd.curl",
    extensions: &[],
    media_types: &["application/vnd.curl"],
    internal_signatures: &[],
    related_formats: &[],
};
