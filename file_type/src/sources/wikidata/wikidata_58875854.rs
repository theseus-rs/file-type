use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58875854: FileFormat = FileFormat {
    id: 58_875_854,
    source_type: SourceType::Wikidata,
    name: "PowerProject",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
