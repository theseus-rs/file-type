use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864300: FileFormat = FileFormat {
    id: 105_864_300,
    puid: "wikidata/105864300",
    name: "Turtle Beach Pinnacle Bank File",
    extensions: &["pbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x69, 0x6E, 0x6E, 0x61, 0x63, 0x6C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x61, 0x6E, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
