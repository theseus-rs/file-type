use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860512: FileFormat = FileFormat {
    id: 105_860_512,
    puid: "wikidata/105860512",
    name: "Alchemy Mindworks Resource data",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4C, 0x43, 0x48, 0x52, 0x53, 0x52, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
