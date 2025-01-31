use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860798: FileFormat = FileFormat {
    id: 105_860_798,
    puid: "wikidata/105860798",
    name: "Allods 2 Rage Of Mages game data archive",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x26, 0x59, 0x41, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
