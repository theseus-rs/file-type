use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866420: FileFormat = FileFormat {
    id: 105_866_420,
    puid: "wikidata/105866420",
    name: "Eclipse Preferences",
    extensions: &["prefs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x65, 0x63, 0x6C, 0x69, 0x70, 0x73, 0x65, 0x2E, 0x70, 0x72, 0x65, 0x66, 0x65,
                    0x72, 0x65, 0x6E, 0x63, 0x65, 0x73, 0x2E, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                    0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
