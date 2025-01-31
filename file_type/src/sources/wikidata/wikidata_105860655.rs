use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860655: FileFormat = FileFormat {
    id: 105_860_655,
    puid: "wikidata/105860655",
    name: "Robinson Technologies Textures",
    extensions: &["rttex"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x54, 0x50, 0x41, 0x43, 0x4B, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
