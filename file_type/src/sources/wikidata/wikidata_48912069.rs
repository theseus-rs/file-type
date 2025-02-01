use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48912069: FileFormat = FileFormat {
    id: 48_912_069,
    puid: "wikidata/48912069",
    name: "IntelliDraw Vector Graphics",
    extensions: &["idw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x69, 0x46, 0x70, 0x53, 0x77, 0x02, 0x77, 0x05, 0x78, 0x00, 0x78, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
