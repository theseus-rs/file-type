use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852983: FileFormat = FileFormat {
    id: 105_852_983,
    puid: "wikidata/105852983",
    name: "Sonnet Project",
    extensions: &["son"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x54, 0x59, 0x50, 0x20, 0x53, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
