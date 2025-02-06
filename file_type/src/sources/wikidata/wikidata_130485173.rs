use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130485173: FileFormat = FileFormat {
    id: 130_485_173,
    source_type: SourceType::Wikidata,
    name: "Portugol file format",
    extensions: &["alg", "portugol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
