use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83159681: FileFormat = FileFormat {
    id: 83_159_681,
    source_type: SourceType::Wikidata,
    name: "RWL",
    extensions: &["rwl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
