use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855099: FileFormat = FileFormat {
    id: 105_855_099,
    puid: "wikidata/105855099",
    name: "Creative Nomad II series MP3 players Voice File audio",
    extensions: &["nvf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x56, 0x46, 0x20, 0x01, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x56,
                    0x46, 0x4D, 0x54, 0x01, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x7D,
                    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x40, 0x1F, 0x00, 0x00, 0x80, 0x3E, 0x00,
                    0x00, 0x02, 0x00, 0x10, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
