use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_64859397: FileFormat = FileFormat {
    id: 64_859_397,
    puid: "wikidata/64859397",
    name: "Family Tree Maker Compressed file format",
    extensions: &["fbc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x61, 0x6D, 0x69, 0x6C, 0x79, 0x20, 0x54, 0x72, 0x65, 0x65, 0x20, 0x4D,
                    0x61, 0x6B, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
