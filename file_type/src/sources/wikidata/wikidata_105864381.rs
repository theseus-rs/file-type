use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864381: FileFormat = FileFormat {
    id: 105_864_381,
    puid: "wikidata/105864381",
    name: "World Construction Set Parameters (v1.x)",
    extensions: &["par"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x47, 0x49, 0x53, 0x50, 0x41, 0x52, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
