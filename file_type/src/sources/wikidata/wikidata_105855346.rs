use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855346: FileFormat = FileFormat {
    id: 105_855_346,
    puid: "wikidata/105855346",
    name: "Foobar 2000 Columns UI settings",
    extensions: &["fcs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xA5, 0xE2, 0xD6, 0x05, 0xD5, 0x9D, 0xCA, 0x23, 0xBE, 0x5A, 0x74, 0x72, 0x3C,
                    0x6F, 0x0C, 0x75,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
