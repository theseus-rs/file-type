use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861731: FileFormat = FileFormat {
    id: 105_861_731,
    puid: "wikidata/105861731",
    name: "MagicEngine savestate",
    extensions: &["me1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x45, 0x43, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
