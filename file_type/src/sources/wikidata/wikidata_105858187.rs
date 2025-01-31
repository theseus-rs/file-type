use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858187: FileFormat = FileFormat {
    id: 105_858_187,
    puid: "wikidata/105858187",
    name: "EZGUI Designer control definition",
    extensions: &["ezc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3C, 0x3C, 0x44, 0x45, 0x53, 0x43, 0x3E, 0x3E, 0x3E, 0x0D, 0x0A, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
