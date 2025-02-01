use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851328: FileFormat = FileFormat {
    id: 105_851_328,
    puid: "wikidata/105851328",
    name: "StarWriter for MS-DOS document (v6)",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x5C, 0x5C, 0x5C, 0x20, 0x57, 0x52, 0x49, 0x54, 0x45, 0x52, 0x20, 0x36,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
