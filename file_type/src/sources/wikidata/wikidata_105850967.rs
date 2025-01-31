use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850967: FileFormat = FileFormat {
    id: 105_850_967,
    puid: "wikidata/105850967",
    name: "NeroAudio Peak file",
    extensions: &["tmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x65, 0x72, 0x6F, 0x41, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x2D, 0x20, 0x50,
                    0x65, 0x61, 0x6B, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
