use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128693745: FileFormat = FileFormat {
    id: 128_693_745,
    source_type: SourceType::Wikidata,
    name: "BNF file",
    extensions: &["bnf"],
    media_types: &["text/x-bnf"],
    internal_signatures: &[],
    related_formats: &[],
};
