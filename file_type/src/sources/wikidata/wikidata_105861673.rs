use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861673: FileFormat = FileFormat {
    id: 105_861_673,
    source_type: SourceType::Wikidata,
    name: "LLVM Bytecode (compressed)",
    extensions: &["bc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x6C, 0x76, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
