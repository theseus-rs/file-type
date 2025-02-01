use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862098: FileFormat = FileFormat {
    id: 105_862_098,
    puid: "wikidata/105862098",
    name: "Miva Compiled script",
    extensions: &["mvc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x69, 0x76, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
