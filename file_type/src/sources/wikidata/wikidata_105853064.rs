use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853064: FileFormat = FileFormat {
    id: 105_853_064,
    puid: "wikidata/105853064",
    name: "Atari ST Guide Hypertext document",
    extensions: &["hyp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x44, 0x4F, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
