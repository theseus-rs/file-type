use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853874: FileFormat = FileFormat {
    id: 105_853_874,
    puid: "wikidata/105853874",
    name: "AWL programming language (Var. 2)",
    extensions: &["awl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x54, 0x41, 0x5F, 0x42, 0x4C, 0x4F, 0x43, 0x4B, 0x20, 0x44, 0x42,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
