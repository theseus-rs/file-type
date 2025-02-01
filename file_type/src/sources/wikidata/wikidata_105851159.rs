use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851159: FileFormat = FileFormat {
    id: 105_851_159,
    puid: "wikidata/105851159",
    name: "PC-88 Tape image",
    extensions: &["t88"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x2D, 0x38, 0x38, 0x30, 0x31, 0x20, 0x54, 0x61, 0x70, 0x65, 0x20,
                    0x49, 0x6D, 0x61, 0x67, 0x65, 0x28, 0x54, 0x38, 0x38, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
