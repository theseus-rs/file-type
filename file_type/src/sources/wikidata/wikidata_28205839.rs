use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205839: FileFormat = FileFormat {
    id: 28_205_839,
    puid: "wikidata/28205839",
    name: "CMU Window Manager bitmap",
    extensions: &["cmu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF1, 0x00, 0x40, 0xBB, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
