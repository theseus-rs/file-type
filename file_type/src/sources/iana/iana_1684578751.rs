use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1684578751: FileFormat = FileFormat {
    id: 1_684_578_751,
    source_type: SourceType::Iana,
    name: "fwdred",
    extensions: &[],
    media_types: &["audio/fwdred"],
    internal_signatures: &[],
    related_formats: &[],
};
