use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858409: FileFormat = FileFormat {
    id: 105_858_409,
    puid: "wikidata/105858409",
    name: "Elastic Reality Shape",
    extensions: &["ers"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x45, 0x6C, 0x61, 0x73, 0x74, 0x69, 0x63, 0x20, 0x52, 0x65, 0x61, 0x6C,
                    0x69, 0x74, 0x79, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
