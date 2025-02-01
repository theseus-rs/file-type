use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857419: FileFormat = FileFormat {
    id: 105_857_419,
    puid: "wikidata/105857419",
    name: "Binary ExtendScript Script",
    extensions: &["jsxbin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x4A, 0x53, 0x58, 0x42, 0x49, 0x4E, 0x40, 0x45, 0x53, 0x40,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
