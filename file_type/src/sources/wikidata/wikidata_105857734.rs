use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857734: FileFormat = FileFormat {
    id: 105_857_734,
    puid: "wikidata/105857734",
    name: "Sound Invasion Music System 2.0 module",
    extensions: &["is20"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x32, 0x30, 0x44, 0x46, 0x31, 0x30, 0x53, 0x54, 0x42, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
