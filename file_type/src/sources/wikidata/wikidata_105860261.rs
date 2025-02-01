use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860261: FileFormat = FileFormat {
    id: 105_860_261,
    puid: "wikidata/105860261",
    name: "PlayStation RSD 3D model info (v3.0)",
    extensions: &["rsd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x52, 0x53, 0x44, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
