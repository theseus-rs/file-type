use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849813: FileFormat = FileFormat {
    id: 105_849_813,
    puid: "wikidata/105849813",
    name: "The Chessmaster 2000 saved game",
    extensions: &["cm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4D, 0x5F, 0x47, 0x61, 0x6D, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
