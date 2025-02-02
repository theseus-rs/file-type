use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2043942: FileFormat = FileFormat {
    id: 2_043_942,
    source_type: SourceType::Wikidata,
    name: "Portable Document Format for Engineering",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
