use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859710: FileFormat = FileFormat {
    id: 105_859_710,
    puid: "wikidata/105859710",
    name: "Visio Drawing (old)",
    extensions: &["vsd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x69, 0x73, 0x69, 0x6F, 0x20, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x44, 0x72,
                    0x61, 0x77, 0x69, 0x6E, 0x67, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
