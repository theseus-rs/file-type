use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857812: FileFormat = FileFormat {
    id: 105_857_812,
    puid: "wikidata/105857812",
    name: "Popcorn Mail configuration",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x6F, 0x70, 0x63, 0x6F, 0x72, 0x6E, 0x53, 0x65, 0x74, 0x74, 0x69,
                    0x6E, 0x67, 0x73, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
