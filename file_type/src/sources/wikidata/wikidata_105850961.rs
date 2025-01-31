use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850961: FileFormat = FileFormat {
    id: 105_850_961,
    puid: "wikidata/105850961",
    name: "MSX Tape image",
    extensions: &["tsx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x54, 0x61, 0x70, 0x65, 0x21, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
