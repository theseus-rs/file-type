use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858419: FileFormat = FileFormat {
    id: 105_858_419,
    puid: "wikidata/105858419",
    name: "Eyeglass format",
    extensions: &["eygl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xBB, 0x0D, 0x0A, 0x65, 0x79, 0x65, 0x67, 0x6C, 0x61, 0x73, 0x73, 0x1A, 0x0A,
                    0xAB,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
