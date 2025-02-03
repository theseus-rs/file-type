use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861425: FileFormat = FileFormat {
    id: 105_861_425,
    source_type: SourceType::Wikidata,
    name: "Lua 5.0 bytecode",
    extensions: &["out"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
