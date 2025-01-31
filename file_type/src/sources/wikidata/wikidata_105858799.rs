use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858799: FileFormat = FileFormat {
    id: 105_858_799,
    puid: "wikidata/105858799",
    name: "LLVM IR Bitcode",
    extensions: &["bc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x43, 0xC0, 0xDE])],
            },
        }],
    }],
    related_formats: &[],
};
