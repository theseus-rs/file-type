use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861397: FileFormat = FileFormat {
    id: 105_861_397,
    puid: "wikidata/105861397",
    name: "LLVM Bytecode",
    extensions: &["bc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x6C, 0x76, 0x6D])],
            },
        }],
    }],
    related_formats: &[],
};
