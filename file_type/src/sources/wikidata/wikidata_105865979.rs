use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865979: FileFormat = FileFormat {
    id: 105_865_979,
    puid: "wikidata/105865979",
    name: "PiXCL text Palette",
    extensions: &["pal"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x69, 0x58, 0x43, 0x4C, 0x2D, 0x50, 0x41, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
