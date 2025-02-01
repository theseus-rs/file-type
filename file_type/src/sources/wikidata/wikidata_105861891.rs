use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861891: FileFormat = FileFormat {
    id: 105_861_891,
    puid: "wikidata/105861891",
    name: "Megatracker module",
    extensions: &["mgt"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x47, 0x54, 0x11, 0xBD, 0x4D, 0x43, 0x53, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
