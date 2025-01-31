use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850753: FileFormat = FileFormat {
    id: 105_850_753,
    puid: "wikidata/105850753",
    name: "Mac OS installable Keyboard Layout (UTF-16 BE)",
    extensions: &["keylayout"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFE, 0xFF, 0x00, 0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C, 0x00,
                    0x20, 0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00, 0x6F,
                    0x00, 0x6E, 0x00, 0x3D, 0x00, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
