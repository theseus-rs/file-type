use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4235591232: FileFormat = FileFormat {
    id: 4_235_591_232,
    source_type: SourceType::Iana,
    name: "vnd.eudora.data",
    extensions: &[],
    media_types: &["application/vnd.eudora.data"],
    internal_signatures: &[],
    related_formats: &[],
};
