use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856613: FileFormat = FileFormat {
    id: 105_856_613,
    puid: "wikidata/105856613",
    name: "Whisper 32 data",
    extensions: &["wsp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x68, 0x69, 0x73, 0x70, 0x65, 0x72, 0x20, 0x33, 0x32, 0x20, 0x46, 0x69,
                    0x6C, 0x65, 0x2C, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
