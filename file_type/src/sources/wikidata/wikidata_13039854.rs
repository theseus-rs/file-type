use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_13039854: FileFormat = FileFormat {
    id: 13_039_854,
    source_type: SourceType::Wikidata,
    name: "C++ header",
    extensions: &["h", "hh", "hpp", "hxx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
