use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858757: FileFormat = FileFormat {
    id: 105_858_757,
    puid: "wikidata/105858757",
    name: "Multipaint image (CPC mode 0)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0xFF, 0x00, 0x02, 0x0F, 0x28, 0x00, 0x19, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
