use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862995: FileFormat = FileFormat {
    id: 105_862_995,
    puid: "wikidata/105862995",
    name: "The Terminator: Dawn of Fate game data archive",
    extensions: &["mpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x41, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
