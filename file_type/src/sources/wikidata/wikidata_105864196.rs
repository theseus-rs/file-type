use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864196: FileFormat = FileFormat {
    id: 105_864_196,
    puid: "wikidata/105864196",
    name: "Test Drive PC shapes game data",
    extensions: &["pes"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x63, 0x6B, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
