use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853930: FileFormat = FileFormat {
    id: 105_853_930,
    source_type: SourceType::Wikidata,
    name: "Atom web feed",
    extensions: &["atom", "xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
