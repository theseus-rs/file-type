use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850089: FileFormat = FileFormat {
    id: 105_850_089,
    puid: "wikidata/105850089",
    name: "TheC64 Config/Joystick/Mode settings (X)",
    extensions: &["cjm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
