use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853930: FileFormat = FileFormat {
    id: 105_853_930,
    source_type: SourceType::Wikidata,
    name: "Atom web feed",
    extensions: &["atom", "xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
