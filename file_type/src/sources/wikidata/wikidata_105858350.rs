use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858350: FileFormat = FileFormat {
    id: 105_858_350,
    puid: "wikidata/105858350",
    name: "GoLabel document",
    extensions: &["ezp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0xC8, 0x42, 0x45, 0x7A, 0x70, 0x6C, 0x20, 0x50, 0x72, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
