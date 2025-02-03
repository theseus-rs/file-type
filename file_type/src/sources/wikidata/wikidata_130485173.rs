use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130485173: FileFormat = FileFormat {
    id: 130_485_173,
    source_type: SourceType::Wikidata,
    name: "Portugol file format",
    extensions: &["alg", "portugol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
